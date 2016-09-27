

- Make ssl work! Use a ssl-test website.
- Make the server a nice process. There should be a way to stop and
  start it in the background, and it should write to a log file.
- Move web page generation to Flask. This will give benefits:
- - No more many-round-trip-post-requests to slow the site down.
- - Can use forms instead of JS buttons. Should give better keyboard
  support.
- - Eliminate all (or almost all) JS on the site, making it much more
  compatible with different browsers. (Only expected difficulty:
  adding/deleting rows from forms.)
- - Should be simpler (less code) overall.
- Bug fixes:
- - Disallow teams with blank names
- - Fix ordering of elements (e.g. hint1, hint2)
- - Test what happens if hint penalties make a score negative.
- Formatting:
- - Align hints.
- - Name table columns consistently (e.g. AST)
- - When submitting a guess, maybe have a popup.
- New pages:
- - View your team's guesses
- - View other teams' guesses, once the hunt is over
- New features:
- - View solutions, once the hunt is over
- - Allow changing your password
- - Allow file uploads?
- - Encouragements.
- - Support multiple hunts (You should be able to view old puzzles,
  hints, solutions, leaderboards, and guesses. You should not be able
  to register a team or submit guesses.
