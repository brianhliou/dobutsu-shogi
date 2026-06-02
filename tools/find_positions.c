/*
 * find_positions.c — mine the clausecker/dobutsu tablebase for facts and
 * showcase positions used in the article:
 *   - the deepest forced win (max distance-to-mate; the paper reports 173)
 *   - a position whose only value-preserving move is an un-advanceable chick
 *     drop onto the enemy back rank (the paper finds 68)
 *   - distance-to-win histogram (depth profile)
 *   - wins where the side to move is behind in material
 *   - distribution of how many winning moves a won position has (how forced)
 *   - the principal variation from the start (the "perfect game")
 *
 * Build it inside the clausecker/dobutsu checkout (needs its headers + objects).
 * See research/reproduction.md. Iteration mirrors validate_tablebase().
 */
#include <stdio.h>
#include <stdlib.h>

#include "dobutsutable.h"

static int
is_unadvanceable_chick_drop(const struct position *p, const struct move *m)
{
	return (m->piece == CHCK_S || m->piece == CHCK_G)
	    && piece_in(HAND, p->pieces[m->piece])
	    && piece_in(PROMZ_S, m->to);
}

/* number of the eight pieces each side currently controls (on board or in hand) */
static void
material(const struct position *p, int *sente, int *gote)
{
	size_t i;

	*sente = *gote = 0;
	for (i = 0; i < PIECE_COUNT; i++)
		if (p->pieces[i] & GOTE_PIECE)
			(*gote)++;
		else
			(*sente)++;
}

/* follow optimal play from the start and print the line (the "perfect game") */
static void
print_perfect_game(const struct tablebase *tb)
{
	struct position p = INITIAL_POSITION;
	char ms[MAX_MOVSTR];
	int ply;

	printf("\n[perfect game from the initial position]\n");
	for (ply = 0; ply < 200; ply++) {
		struct move moves[MAX_MOVES], best;
		struct position bestp;
		size_t i, nmove;
		tb_entry bestval = 1;
		int ended = 0, found = 0;

		nmove = generate_moves(moves, &p);
		if (nmove == 0)
			break;

		for (i = 0; i < nmove; i++) {
			struct position pp = p;
			tb_entry e;

			if (play_move(&pp, moves + i)) {   /* move ends the game: immediate win */
				best = moves[i]; bestp = pp; ended = 1; found = 1;
				break;
			}
			e = lookup_position(tb, &pp);
			if (!found || wdl_compare(bestval, e) >= 0) {
				bestval = e; best = moves[i]; bestp = pp; found = 1;
			}
		}

		move_string(ms, &p, &best);
		printf("%d. %s\n", ply + 1, ms);
		p = bestp;
		if (ended)
			break;
	}
	printf("(%d plies)\n", ply + 1);
}

int
main(int argc, char *argv[])
{
	FILE *f;
	struct tablebase *tb;
	poscode pc;

	long long scanned = 0, wins = 0, n173 = 0, ndrop = 0, ndrop_win = 0, behind = 0;
	int maxdtm = 0, i;
	long long dtmhist[256] = {0}, forcing[64] = {0};
	struct position p173, pdrop;
	struct move mdrop;
	tb_entry e173 = 0, edrop = 0;
	int have173 = 0, havedrop = 0;
	char ps[MAX_POSSTR], rnd[MAX_RENDER], ms[MAX_MOVSTR];

	if (argc != 2) { fprintf(stderr, "usage: %s game.tb\n", argv[0]); return (EXIT_FAILURE); }
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
						continue;
					e = lookup_position(tb, &p);
					scanned++;

					if (is_win(e)) {
						struct move moves[MAX_MOVES], wm;
						size_t k, nmove;
						int dtm = get_dtm(e), winmoves = 0, sm, gm;

						wins++;
						if (dtm > maxdtm) maxdtm = dtm;
						if (dtm < 256) dtmhist[dtm]++;
						if (dtm == 173) { n173++; if (!have173) { p173 = p; e173 = e; have173 = 1; } }

						material(&p, &sm, &gm);
						if (sm < gm) behind++;

						nmove = generate_moves(moves, &p);
						for (k = 0; k < nmove; k++) {
							struct position pp = p;
							play_move(&pp, moves + k);
							if (is_loss(lookup_position(tb, &pp))) { winmoves++; wm = moves[k]; }
						}
						if (winmoves < 64) forcing[winmoves]++;
						if (winmoves == 1 && is_unadvanceable_chick_drop(&p, &wm)) {
							ndrop++; ndrop_win++;
							if (!havedrop) { pdrop = p; mdrop = wm; edrop = e; havedrop = 1; }
						}
					} else if (is_draw(e)) {
						struct move moves[MAX_MOVES], dm;
						size_t k, nmove;
						int keep = 0;

						nmove = generate_moves(moves, &p);
						for (k = 0; k < nmove; k++) {
							struct position pp = p;
							play_move(&pp, moves + k);
							if (is_draw(lookup_position(tb, &pp))) { keep++; dm = moves[k]; if (keep > 1) break; }
						}
						if (keep == 1 && is_unadvanceable_chick_drop(&p, &dm))
							ndrop++;
					}
				}
		}

	printf("scanned non-terminal positions = %lld\n", scanned);
	printf("win positions                  = %lld\n", wins);
	printf("max distance-to-win (plies)    = %d\n", maxdtm);
	printf("positions with dtm == 173      = %lld\n", n173);
	printf("only-move = un-advanceable chick drop = %lld (wins: %lld)\n", ndrop, ndrop_win);
	printf("wins where side to move is behind in material = %lld (%.1f%% of wins)\n",
	    behind, 100.0 * behind / wins);

	printf("\n[forcing: won positions by number of winning moves]\n");
	for (i = 0; i < 64; i++)
		if (forcing[i])
			printf("  %2d winning move(s): %lld\n", i, forcing[i]);

	printf("\n[depth profile: won positions by distance-to-win, plies]\n");
	for (i = 0; i < 256; i++)
		if (dtmhist[i])
			printf("  dtm %3d: %lld\n", i, dtmhist[i]);

	if (have173) {
		position_string(ps, &p173); position_render(rnd, &p173);
		printf("\n[173-ply win] %s\n%s\n", ps, rnd);
	}
	if (havedrop) {
		position_string(ps, &pdrop); move_string(ms, &pdrop, &mdrop); position_render(rnd, &pdrop);
		printf("[chick-drop-only %s] %s  only move: %s\n%s\n",
		    is_win(edrop) ? "win" : "draw", ps, ms, rnd);
	}

	print_perfect_game(tb);
	return (EXIT_SUCCESS);
}
