<?xml version="1.0" encoding="UTF-8"?>

<xsl:transform version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

  <xsl:strip-space elements="*"/>

  <xsl:template match="title">
    <h1><xsl:copy-of select="node()"/></h1>
  </xsl:template>

  <xsl:template match="subtitle">
    <p class="subtitle">
      <xsl:copy-of select="node()"/>
    </p>
  </xsl:template>

  <xsl:template match="content">
    <xsl:copy-of select="node()"/>
  </xsl:template>

  <xsl:template match="puzzle">
    <p class="subtitle">This is a puzzle.</p>
    <xsl:copy-of select="description/node()"/>
    <p><a href="/guess.xml?puzzle={id}">Submit an answer</a></p>
  </xsl:template>

  <xsl:template match="form">
    <script type="text/javascript" defer="defer">
      document.addEventListener("DOMContentLoaded", function() {
        var dropdown = document.getElementById("puzzle_select");
        setDropdownOption(dropdown, QUERY['puzzle']);
      });
    </script>
    <form action="{action}">
      <xsl:for-each select="input">
        <xsl:if test="contains(@type, 'text')">
          <p>
            <xsl:value-of select="normalize-space(.)"/>:
            <br/>
            <input type="text" name="{@id}"/>
          </p>
        </xsl:if>
        <xsl:if test="contains(@type, 'number')">
          <p>
            <xsl:value-of select="normalize-space(.)"/>:
            <br/>
            <input type="text" name="{@id}"/>
          </p>
        </xsl:if>
        <xsl:if test="contains(@type, 'puzzle')">
          <p>
            <xsl:value-of select="normalize-space(.)"/>:
            <br/>
            <select id="puzzle_select">
              <xsl:for-each select="document('hunt.xml')//puzzle[@id]">
                <option value="{@id}">
                  <xsl:value-of select="@name"/>
                </option>
              </xsl:for-each>
              <option value="" selected="selected">
                Select a puzzle
              </option>
            </select>
            </p>
        </xsl:if>
      </xsl:for-each>
      <input type="submit" value="{description}"/>
    </form>
  </xsl:template>

  <xsl:template match="list-all-puzzles">
    <ul style="list-style-type: none">
      <xsl:for-each select="document('hunt.xml')/hunt/all-puzzles/*">
        <xsl:call-template name="list-a-puzzles"/>
      </xsl:for-each>
    </ul>
  </xsl:template>

  <xsl:template name="list-a-puzzles">
    <xsl:if test="name() = 'puzzles'">
      <li>
        <b><xsl:value-of select="@name"/>:</b>
        <ul>
          <xsl:for-each select="*">
            <xsl:call-template name="list-a-puzzles"/>
          </xsl:for-each>
        </ul>
      </li>
    </xsl:if>
    <xsl:if test="name() = 'puzzle'">
      <li>
        <a href="puzzles/{@id}.xml">
          <xsl:value-of select="@name"/>
        </a>
      </li>
    </xsl:if>
  </xsl:template>

  <xsl:template name="Header">
    <xsl:variable name="hunt-name" select="document('hunt.xml')/hunt/name"/>
    <xsl:variable name="puzzle-list">
      <xsl:for-each select="document('hunt.xml')//puzzle[@id]/@id">
        <xsl:value-of select="."/>
        <xsl:text>,</xsl:text>
      </xsl:for-each>
    </xsl:variable>
    <title>
      <xsl:value-of select="$hunt-name"/>
    </title>
    <link rel="stylesheet" type="text/css" href="../css/style.css"/>
    <script type="text/javascript">
      var PUZZLES = '<xsl:value-of select="$puzzle-list"/>'.split(",");
      PUZZLES.pop();
    </script>
    <script type="text/javascript" src="/ph.js"/>
  </xsl:template>

  <xsl:template match="page">
    <xsl:variable name="hunt-name" select="document('hunt.xml')/hunt/name"/>
    <html>
      <head>
        <xsl:call-template name="Header"/>
      </head>
      <body>
        <ul class="nav">
          <li class="nav1">
            <a href="/index.xml">
              <xsl:value-of select="$hunt-name"/>
            </a>
          </li>
          <li class="nav2"><a href="/team.xml">Team</a></li>
          <li class="nav3"><a href="/leaderboard.xml">Leaderboard</a></li>
          <li class="nav4"><a href="/puzzles.xml">Puzzles</a></li>
        </ul>
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
  
  <xsl:template match="master-page">
    <html>
      <head>
        <xsl:call-template name="Header"/>
      </head>
      <body>
        <ul class="nav">
          <li class="nav1"><a href="/master/hunt.xml">Hunt</a></li>
          <li class="nav2"><a href="/master/puzzles.xml">Puzzles</a></li>
          <li class="nav3"><a href="/master/hints.xml">Hints</a></li>
          <li class="nav4"><a href="/master/waves.xml">Waves</a></li>
        </ul>
        <article>
          <xsl:apply-templates select="*"/>
          <footer>
            * Welcome, master. *
          </footer>
        </article>
      </body>
    </html>
  </xsl:template>
  
</xsl:transform>
