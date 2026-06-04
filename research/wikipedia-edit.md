# Wikipedia correction — Dōbutsu shōgi "Theoretical result"

Target: <https://en.wikipedia.org/wiki/D%C5%8Dbutsu_sh%C5%8Dgi> · section **Theoretical result**.
Apply under your own account. This is a primary-source-grounded correction of a confirmed error.

## The error

The article's opening sentence of *Theoretical result* states:

> ...the theoretical best move from each reachable position **(there are 1,567,925,964 reachable
> positions in the game)** of the game is known.

**1,567,925,964 is not the reachable count.** Per Tanaka (2009), it is the upper bound on *all*
arrangements of the pieces ignoring reachability (Table 1 sum). The number of positions actually
**reachable** from the starting position is **246,803,167** (§3.1). The body also never names the
solver, year, or method — all worth adding.

## Proposed edit (wikitext)

**Replace** the first sentence of the section:

```wikitext
Dōbutsu shōgi has been [[solved game|strongly solved]], meaning that the theoretical best move from each reachable position (there are 1,567,925,964 reachable positions in the game) of the game is known.
```

**with:**

```wikitext
Dōbutsu shōgi was [[solved game|strongly solved]] in 2009 by Tetsuro Tanaka, who used [[retrograde analysis]] to determine the optimal outcome from every position reachable from the starting setup. There are 246,803,167 such reachable positions; a larger figure of 1,567,925,964 sometimes given for the game is an upper bound on all arrangements of the pieces, including positions that never arise in play.<ref name="dobutsu-solved" />
```

(The rest of the paragraph — the second-player win, 78 plies, zugzwang/Trébuchet — is correct and
stays unchanged.)

## Optional: upgrade the citation (recommended, low risk)

The existing reference is a bare title+URL. Replace the **full definition** of `dobutsu-solved`
(currently on the "All opening moves…" sentence) with a proper conference-paper citation:

```wikitext
<ref name="dobutsu-solved">{{Cite journal |last=Tanaka |first=Tetsuro |script-title=ja:「どうぶつしょうぎ」の完全解析 |trans-title=An Analysis of a Board Game "Doubutsu Shogi" |journal=IPSJ SIG Technical Reports |volume=2009-GI-22 |issue=3 |pages=1–8 |year=2009 |publisher=Information Processing Society of Japan |language=ja |url=https://www.tanaka.ecc.u-tokyo.ac.jp/ktanaka/dobutsushogi/ |id=[http://id.nii.ac.jp/1001/00062415/ NII permalink]}}</ref>
```

Keep the self-closing `<ref name="dobutsu-solved" />` use in the first paragraph; both resolve to
this one definition.

## Edit summary (paste into the summary box)

```
Theoretical result: fix position count — 1,567,925,964 is the all-arrangements upper bound (Tanaka 2009, Table 1); reachable positions = 246,803,167 (§3.1). Add solver/year/method attribution; upgrade citation.
```

## Talk-page note (post to Talk:Dōbutsu shōgi to preempt a revert)

```
== Position count corrected (1,567,925,964 → 246,803,167) ==

The "Theoretical result" section stated there are 1,567,925,964 reachable positions. That figure
is actually the upper bound on all piece arrangements ignoring reachability — the sum of Table 1 in
Tanaka's 2009 paper (the article's own cited source). The number of positions reachable from the
starting position is 246,803,167 (paper §3.1). I've corrected the figure, kept the (correct) 78-ply
second-player-win result, added attribution to Tetsuro Tanaka (2009) and the retrograde-analysis
method, and upgraded the citation to a full reference. Both numbers trace to the same primary
source. ~~~~
```

## Sources

- Tetsuro Tanaka, "An Analysis of a Board Game 'Doubutsu Shogi'" (「どうぶつしょうぎ」の完全解析),
  IPSJ SIG Technical Reports, Vol. 2009-GI-22, No. 3, pp. 1–8 (2009).
  NII: <http://id.nii.ac.jp/1001/00062415/> · paper: §2/Table 1 (upper bound), §3.1 (reachable).
- Our independent reproduction: `research/reproduction.md` (initial `#-78`, validated vs clausecker).
