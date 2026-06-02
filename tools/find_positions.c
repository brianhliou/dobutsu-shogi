/*
 * find_positions.c — scan the clausecker/dobutsu tablebase for the article's
 * two showcase positions:
 *   1. the deepest forced win (max distance-to-mate; the paper reports 173 plies)
 *   2. a position whose only value-preserving move is an un-advanceable chick
 *      drop onto the enemy back rank (the paper finds 68 such positions)
 *
 * Build it inside the clausecker/dobutsu checkout (it needs that project's
 * headers and compiled objects). See research/reproduction.md.
 *
 * Iteration mirrors validate_tablebase() in tbvalidate.c.
 */
#include <stdio.h>
#include <stdlib.h>

#include "dobutsutable.h"

/* a chick dropped onto the mover's far rank (PROMZ_S) can never advance */
static int
is_unadvanceable_chick_drop(const struct position *p, const struct move *m)
{
	return (m->piece == CHCK_S || m->piece == CHCK_G)
	    && piece_in(HAND, p->pieces[m->piece])
	    && piece_in(PROMZ_S, m->to);
}

int
main(int argc, char *argv[])
{
	FILE *f;
	struct tablebase *tb;
	poscode pc;

	long long scanned = 0, wins = 0, n173 = 0, ndrop = 0, ndrop_win = 0;
	int maxdtm = 0;
	struct position p173, pdrop, pmax;
	struct move mdrop;
	tb_entry e173 = 0, edrop = 0, emax = 0;
	int have173 = 0, havedrop = 0, havemax = 0;
	char ps[MAX_POSSTR], rnd[MAX_RENDER], ms[MAX_MOVSTR];

	if (argc != 2) {
		fprintf(stderr, "usage: %s game.tb\n", argv[0]);
		return (EXIT_FAILURE);
	}
	f = fopen(argv[1], "rb");
	if (f == NULL) { perror("fopen"); return (EXIT_FAILURE); }
	tb = read_tablebase(f);
	if (tb == NULL) { perror("read_tablebase"); return (EXIT_FAILURE); }

	for (pc.ownership = 0; pc.ownership < OWNERSHIP_TOTAL_COUNT; pc.ownership++)
		for (pc.cohort = 0; pc.cohort < COHORT_COUNT; pc.cohort++) {
			unsigned size;

			if (!has_valid_ownership(pc))
				continue;

			size = cohort_size[pc.cohort].size;
			for (pc.lionpos = 0; pc.lionpos < LIONPOS_COUNT; pc.lionpos++)
				for (pc.map = 0; pc.map < size; pc.map++) {
					struct position p;
					tb_entry e;

					decode_poscode(&p, pc);
					if (gote_in_check(&p))
						continue;  /* terminal win-determined position */

					e = lookup_position(tb, &p);
					scanned++;

					if (is_win(e)) {
						int dtm = get_dtm(e);

						wins++;
						if (dtm > maxdtm) {
							maxdtm = dtm; pmax = p; emax = e; havemax = 1;
						}
						if (dtm == 173) {
							n173++;
							if (!have173) { p173 = p; e173 = e; have173 = 1; }
						}
					}

					/* only-value-preserving-move test (win or draw) */
					if (is_win(e) || is_draw(e)) {
						struct move moves[MAX_MOVES], pm;
						size_t i, nmove;
						int preserve = 0;

						nmove = generate_moves(moves, &p);
						for (i = 0; i < nmove; i++) {
							struct position pp = p;
							tb_entry ce;

							play_move(&pp, moves + i);
							ce = lookup_position(tb, &pp);
							if ((is_win(e) && is_loss(ce)) ||
							    (is_draw(e) && is_draw(ce))) {
								preserve++;
								pm = moves[i];
								if (preserve > 1)
									break;
							}
						}

						if (preserve == 1 && is_unadvanceable_chick_drop(&p, &pm)) {
							ndrop++;
							if (is_win(e))
								ndrop_win++;
							if (!havedrop || (is_win(e) && !is_win(edrop))) {
								pdrop = p; mdrop = pm; edrop = e; havedrop = 1;
							}
						}
					}
				}
		}

	printf("scanned non-terminal positions = %lld\n", scanned);
	printf("win positions                  = %lld\n", wins);
	printf("max distance-to-win (plies)    = %d\n", maxdtm);
	printf("positions with dtm == 173      = %lld\n", n173);
	printf("only-move = un-advanceable chick drop = %lld (wins: %lld)\n", ndrop, ndrop_win);

	if (have173) {
		position_string(ps, &p173);
		position_render(rnd, &p173);
		printf("\n[173-ply win] %s\n%s\n", ps, rnd);
	}
	if (havemax && maxdtm != 173) {
		position_string(ps, &pmax);
		position_render(rnd, &pmax);
		printf("[max-dtm=%d] %s\n%s\n", get_dtm(emax), ps, rnd);
	}
	if (havedrop) {
		position_string(ps, &pdrop);
		move_string(ms, &pdrop, &mdrop);
		position_render(rnd, &pdrop);
		printf("[chick-drop-only %s] %s  only move: %s\n%s\n",
		    is_win(edrop) ? "win" : "draw", ps, ms, rnd);
	}

	return (EXIT_SUCCESS);
}
