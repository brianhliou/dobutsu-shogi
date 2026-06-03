/*
 * probe.c — a stdin/stdout JSON probe over the clausecker/dobutsu tablebase,
 * for the explorer's "lichess-style" move panel. Loads the tablebase once,
 * then for each position string read on a line prints one JSON line:
 *
 *   {"pos":"...","side":"S","value":{"result":"loss","dtm":78},
 *    "moves":[{"move":"Gc4-c3","result":"loss","dtm":78,"to":"G/..."} , ...]}
 *
 * result/dtm are from the side-to-move's point of view; dtm is in plies.
 * A move's "to" is the resulting position string (empty if the move ends the
 * game by capturing the lion or completing a Try).
 *
 * Build inside the clausecker checkout; see explorer/README.md.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dobutsutable.h"

/* strip spaces that move_string pads drops with ("C  *b1" -> "C*b1") */
static void
squash(char *s)
{
	char *w = s;
	for (; *s; s++)
		if (*s != ' ')
			*w++ = *s;
	*w = '\0';
}

static const char *
result_of(tb_entry e)
{
	return is_win(e) ? "win" : is_loss(e) ? "loss" : "draw";
}

int
main(int argc, char *argv[])
{
	FILE *f;
	struct tablebase *tb;
	char line[64];

	if (argc != 2) { fprintf(stderr, "usage: %s game.tb\n", argv[0]); return (EXIT_FAILURE); }
	f = fopen(argv[1], "rb");
	if (f == NULL) { perror("fopen"); return (EXIT_FAILURE); }
	tb = read_tablebase(f);
	if (tb == NULL) { perror("read_tablebase"); return (EXIT_FAILURE); }

	setvbuf(stdout, NULL, _IOLBF, 0);

	while (fgets(line, sizeof line, stdin) != NULL) {
		struct position p;
		struct move moves[MAX_MOVES];
		size_t i, nmove;
		tb_entry v;

		line[strcspn(line, "\r\n")] = '\0';
		if (parse_position(&p, line) != 0) {
			printf("{\"error\":\"bad position\"}\n");
			continue;
		}

		v = lookup_position(tb, &p);
		printf("{\"pos\":\"%s\",\"side\":\"%c\",\"value\":{\"result\":\"%s\",\"dtm\":%d},\"moves\":[",
		    line, gote_moves(&p) ? 'G' : 'S', result_of(v), get_dtm(v));

		nmove = generate_moves(moves, &p);
		for (i = 0; i < nmove; i++) {
			struct position pp = p;
			char ms[MAX_MOVSTR], to[MAX_POSSTR];
			const char *res;
			int dtm, ge;

			move_string(ms, &p, &moves[i]);
			squash(ms);
			ge = play_move(&pp, &moves[i]);
			if (ge) {            /* move ends the game: immediate win for the mover */
				res = "win"; dtm = 1; to[0] = '\0';
			} else {
				tb_entry e = lookup_position(tb, &pp);
				res = is_loss(e) ? "win" : is_win(e) ? "loss" : "draw";
				dtm = is_draw(e) ? 0 : get_dtm(e) + 1;
				position_string(to, &pp);
			}
			printf("%s{\"move\":\"%s\",\"result\":\"%s\",\"dtm\":%d,\"to\":\"%s\"}",
			    i ? "," : "", ms, res, dtm, to);
		}
		printf("]}\n");
	}

	return (EXIT_SUCCESS);
}
