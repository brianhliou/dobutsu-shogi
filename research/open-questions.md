# Open questions

The backlog. Each gets resolved by a source or by our own experiment — not by guessing in
prose. Move resolved items into `findings.md` with their source.

## Provenance / citation

- [ ] **Exact venue + volume/pages.** Is it IPSJ SIG-GI Technical Report 2009-GI-22(?), and
      what page range? One search said "Vol. 2009-GI-22, No. 3, pp. 1–8"; another implied GPW.
      The 2009-06-26 slide date points to a June SIG-GI meeting, not the November GPW. Pin the
      canonical citation from Tanaka's own publications page
      (https://www.tanaka.ecc.u-tokyo.ac.jp/ktanaka/papers.html) or CiNii/J-GLOBAL.
- [ ] **Is there a later/journal version?** The PDF metadata says 2 pages but it renders 8;
      confirm we have the full report and not an extended abstract with a separate long form.

## Facts to nail before publishing

- [ ] **Initial setup orientation.** Confirm A4 Elephant / C4 Giraffe (sente) against the
      Fig. 1 image and the official rules. (Move-list logic already pins giraffe→C4.)
- [ ] **Wikipedia "best opening move = capture chick" wording.** Our read of §4.1 is that the
      chick capture *loses fastest* (76 plies). Confirm the exact Wikipedia sentence and
      whether it's stating "delays loss least" (correct) or "best move" (misleading).
- [ ] **"Strongly solved" vs "solved for reachable positions."** Decide the precise claim.
      Tanaka solved reachable positions; does clausecker's tablebase cover *all* legal
      positions (true strong solution)? Check the clausecker/dobutsu and nodan/dobutsu repos.
- [ ] **Plies vs moves consistency.** Confirm 手 = ply throughout (78, 173, 23 are all ply
      counts). Table 3's odd-only mate lengths support ply-counting.

## Deeper / for our own experiments

- [ ] **Reproduce the headline.** Build or run an existing solver to independently confirm the
      gote-win-in-78 and the 246,803,167 reachable count. Candidates: clausecker/dobutsu (C
      tablebase), nodan/dobutsu. This is the "we re-ran it ourselves" credibility beat.
- [ ] **Verify 173-ply max and the 68 chick-drop positions** from a tablebase, if feasible.
- [ ] **Drops ablation, quantified.** Tanaka shows forbidding enemy-zone chick-drops changes
      4,301 positions. The bigger design question (from the chess note): how much shallower is
      the game with *no drop rule at all*? That's our own experiment, not in the paper.
- [ ] **Goro Goro / extended variants.** 5×6 "Goro Goro Dōbutsu Shōgi" and the 拡張どうぶつ将棋
      analyses (e.g. the 2013 Kindai University thesis). Solved? By whom? Different result?

## Narrative / framing (for article/outline.md)

- [ ] Best one-sentence intuition for *why* the second player wins (zugzwang framing).
- [ ] How much shogi-drops explanation does a general/chess audience need before the payoff?
- [ ] Lead with the "Wikipedia is wrong" correction, or earn it mid-piece? (Tone risk.)
