<?xml version="1.0" encoding="UTF-8"?>

<xsl:transform version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

  <xsl:strip-space elements="*"/>

  
  <!--******************** UTILITY ********************-->
  
  <xsl:template match="title">
    <h1 id="title"><xsl:copy-of select="node()"/></h1>
  </xsl:template>

  <xsl:template match="subtitle">
    <p id="subtitle" class="subtitle">
      <xsl:copy-of select="node()"/>
    </p>
  </xsl:template>

  <xsl:template match="content">
    <xsl:copy-of select="node()"/>
  </xsl:template>

  <xsl:template match="puzzle">
    <p class="subtitle">This is a puzzle.</p>
    <xsl:copy-of select="description/node()"/>
    <p><a href="/guess.xml?puzzle={name}">Submit an answer</a></p>
  </xsl:template>

  
  <!--******************** LIST ALL PUZZLES ********************-->

  <xsl:template match="list-all-puzzles">
    <div id="all-puzzles"/>
  </xsl:template>

  
  <!--******************** PAGES ********************-->
  
  <xsl:template name="Header">
    <title>CRUMS 6102</title>
    <link rel="stylesheet" type="text/css" href="/css/style.css"/>
    <script type="text/javascript">
      var PUZZLES = [];
      PUZZLES.pop();
    </script>
    <script type="text/javascript" src="/ph.js"/>
  </xsl:template>

  <xsl:template match="page">
    <html>
      <head>
        <xsl:call-template name="Header"/>
        <script type="text/javascript">
          window.addEventListener("load", function() {
            performAction('<xsl:value-of select="@action"/>')
          });
        </script>
      </head>
      <body>
        <ul class="nav">
          <li class="nav1">
            <a href="/index.xml" id="hunt-title">CRUMS 6102</a>
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
  
  <xsl:template match="master-page">
    <html>
      <head>
        <xsl:call-template name="Header"/>
        <script type="text/javascript">
          window.addEventListener("load", function() {
            performAction('<xsl:value-of select="@action"/>')
          });
        </script>
      </head>
      <body>
        <ul class="nav">
          <li class="nav1"><a href="/master/hunt.xml">Hunt</a></li>
          <li class="nav2"><a href="/master/puzzles.xml">Puzzles</a></li>
          <li class="nav3"><a href="/master/hints.xml">Hints</a></li>
          <li class="nav4"><a href="/master/waves.xml">Waves</a></li>
          <li class="nav5"><a href="/master/members.xml">Members</a></li>
          <li class="nav6"><a href="/master/logout.xml">Logout</a></li>
        </ul>
        <p id="success-message"/>
        <p id="failure-message"/>
        <article>
          <xsl:apply-templates select="*"/>
          <footer>
            * Welcome, Master. *
          </footer>
        </article>
      </body>
    </html>
  </xsl:template>

  
  <!--******************** FORMS ********************-->

  <xsl:template name="WaveInput">
    <select name="wave">
      <option value="" selected="selected" disabled="true">
        Select a wave
      </option>
    </select>
  </xsl:template>

  <xsl:template name="PuzzleInput">
    <select name="puzzle">
      <option value="" selected="selected" disabled="true">
        Select a puzzle
      </option>
    </select>
  </xsl:template>

  <xsl:template match="table">
    <p>
      <table class="grid">
        <tbody id="table">
          <tr>
            <xsl:for-each select="column">
              <th id="{@id}">
                <xsl:value-of select="."/>
              </th>
            </xsl:for-each>
          </tr>
        </tbody>
      </table>
    </p>
  </xsl:template>

  <xsl:template match="multi-form">
    <p>
      <table>
        <xsl:for-each select="input">
          <tr>
            <td><b><xsl:value-of select="@name"/>:</b></td>
            <td><xsl:value-of select="."/></td>
          </tr>
        </xsl:for-each>
      </table>
    </p>
    <p>
      <table>
        <tbody id="multi-form" action="{@action}">
          <tr>
            <xsl:for-each select="input">
              <th><xsl:value-of select="normalize-space(@name)"/></th>
            </xsl:for-each>
          </tr>
          <tr id="row-template" style="display:none">
            <xsl:for-each select="input">
              <td>
                <xsl:if test="contains(@type, 'short-text')">
                  <input type="text"
                         name="{@id}"
                         class="multi-form-cell short-text"/>
                </xsl:if>
                <xsl:if test="contains(@type, 'number')">
                  <input type="text"
                         name="{@id}"
                         class="multi-form-cell number"/>
                </xsl:if>
                <xsl:if test="contains(@type, 'long-text')">
                  <input type="text"
                         name="{@id}"
                         class="multi-form-cell long-text"/>
                </xsl:if>
                <xsl:if test="contains(@type, 'fixed')">
                  <input type="text"
                         name="{@id}"
                         disabled="true"
                         class="multi-form-cell fixed-text"/>
                </xsl:if>
                <xsl:if test="contains(@type, 'datetime')">
                  <input type="datetime-local"
                         name="{@id}"
                         class="multi-form-cell datetime"
                         step="1"/>
                </xsl:if>
                <xsl:if test="contains(@type, 'puzzle')">
                  <xsl:call-template name="PuzzleInput"/>
                </xsl:if>
                <xsl:if test="contains(@type, 'wave')">
                  <xsl:call-template name="WaveInput"/>
                </xsl:if>
              </td>
            </xsl:for-each>
            <td>
              <a href="#" onclick="deleteRow(this)">Delete</a>
            </td>
          </tr>
        </tbody>
      </table>
      <a href="#" onclick="addRow(undefined, '{@id}')">
        Add <xsl:value-of select="@item"/>
      </a>
    </p>
    <xsl:for-each select="submit-button">
      <input type="button"
             value="{normalize-space(.)}"
             onclick="performAction('{@action}', '{@id}')"/>
      <br/>
    </xsl:for-each>
  </xsl:template>

  <xsl:template match="form">
    <table>
      <tbody id="form" action="{@action}">
        <xsl:for-each select="input">
          <tr>
            <xsl:if test="contains(@type, 'password')">
              <td><xsl:value-of select="normalize-space(.)"/>:</td>
              <td><input type="password"
                         name="{@id}"
                         class="form-cell text"/></td>
            </xsl:if>
            <xsl:if test="contains(@type, 'text')">
              <td><xsl:value-of select="normalize-space(.)"/>:</td>
              <td><input type="text"
                         name="{@id}"
                         class="form-cell text"/></td>
            </xsl:if>
            <xsl:if test="contains(@type, 'number')">
              <td><xsl:value-of select="normalize-space(.)"/>:</td>
              <td><input type="text"
                         name="{@id}"
                         class="form-cell number"/></td>
            </xsl:if>
            <xsl:if test="contains(@type, 'fixed')">
              <td><xsl:value-of select="normalize-space(.)"/>:</td>
              <td><input type="text"
                         disabled="true"
                         name="{@id}"
                         class="form-cell text"/></td>
            </xsl:if>
            <xsl:if test="contains(@type, 'boolean')">
              <td><xsl:value-of select="normalize-space(.)"/>:</td>
              <td><input type="checkbox"
                         name="{@id}"
                         class="form-cell checkbox"/></td>
            </xsl:if>
            <xsl:if test="contains(@type, 'puzzle')">
              <td><xsl:value-of select="normalize-space(.)"/>:</td>
              <td><xsl:call-template name="PuzzleInput"/></td>
            </xsl:if>
          </tr>
        </xsl:for-each>
      </tbody>
    </table>
    <xsl:for-each select="submit-button">
      <input type="button"
             value="{normalize-space(.)}"
             onclick="performAction('{@action}')"/>
      <br/>
    </xsl:for-each>
  </xsl:template>
  
</xsl:transform>
