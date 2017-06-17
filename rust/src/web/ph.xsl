<?xml version="1.0" encoding="UTF-8"?>

<xsl:transform version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  
  <xsl:template match="hunt">
    <h1><xsl:copy-of select="node()"/></h1>
  </xsl:template>

  <!-- Puzzle List -->
  <xsl:template match="puzzle">
    <li>
      <a href="puzzles/{@key}.xml"><xsl:value-of select="@name"/></a>
    </li>
  </xsl:template>

  <xsl:template match="wave">
    <p>
      <xsl:value-of select="@name"/>:
      <ul class="puzzle-list"><xsl:apply-templates select="*"/></ul>
    </p>
  </xsl:template>

  <xsl:template match="waves">
    <h2>Puzzles</h2>
    <xsl:apply-templates select="*"/>
  </xsl:template>


<!--
    sublist.classList.add("puzzle-list");
    for (var j = 0; j < puzzles.length; j++) {
      var puzzle = puzzles[j];
      if (puzzle.wave === wave) {
        name = puzzle.number ? puzzle.number + ": " + puzzle.name : puzzle.name;
        var link = make("a", {
          "textContent": name,
          "href": "puzzles/" + puzzle.key + ".xml"
        });
        var elem = make("li");
        elem.appendChild(link);
        for (var k = 0; k < puzzle.hints.length; k++) {
          var hint = puzzle.hints[k];
          elem.appendChild(make("span", {"className": "spacing"}));
          elem.appendChild(make("a", {
            "textContent": "Hint " + hint.number,
            "href":        "hints/" + hint.key + ".xml"
          }));
        }
        sublist.appendChild(elem);
      }
    }
    var waveli = <p>wave:
      "textContent": wave + ":"
    });
    waveli.appendChild(sublist);
    list.appendChild(waveli);
  }
-->
  
  <!-- Page Template -->

  <xsl:template match="page">
    <html>
      <head>
        <title><xsl:value-of select="hunt"/></title>
        <link rel="stylesheet" type="text/css" href="/css/style.css"/>
      </head>
      <body>
        <ul class="nav">
          <li class="nav1">
            <a href="/index.xml"><xsl:value-of select="hunt"/></a>
          </li>
          <li class="nav2"><a href="/team.xml">Team</a></li>
          <li class="nav3"><a href="/team-leaderboard.xml">Leaderboard</a></li>
          <li class="nav4"><a href="/puzzle-leaderboard.xml">Puzzle Stats</a></li>
          <li class="nav5"><a href="/puzzles.xml">Puzzles</a></li>
        </ul>
        <p id="success-message"/>
        <p id="failure-message"/>
        <article>
          <xsl:apply-templates select="*"/>
          <footer>
            <a style="text-decoration: none"
               href="https://github.com/justinpombrio/PuzzleHunt-PH">
              * Made with Puzzle Hunt: PH *
            </a>
          </footer>
        </article>
      </body>
    </html>
  </xsl:template>
  
</xsl:transform>

