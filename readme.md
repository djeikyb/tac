# tac

Kinda like cat, but opposite, and with hardly any features.
Why?
Because I didn't want to figure out which brew command would bring in a million other utilities plus tac.

## use

I want to look at my git graph, then copy a block of commits to paste into an interactive rebase.
Unfortunately, you can't use the reverse flag together with graph.
If you expand and simplify my git-ll alias that produces a fancy git log on the terminal,
my sole need for tac is basically this:

    git log --oneline --graph head~5..head^ | tac | pbcopy

## build

    dotnet publish -c release

## install

    ln artifacts/publish/tac/release/tac ~/bin/tac
