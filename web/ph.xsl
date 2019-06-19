<?xml version="1.0" encoding="UTF-8"?>

<xsl:transform version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

  <xsl:strip-space elements="*"/>
  
  <xsl:template match="hunt">
  </xsl:template>
  
  <xsl:template match="title">
    <h1><xsl:copy-of select="node()"/></h1>
  </xsl:template>
  
  <xsl:template match="content">
    <xsl:copy-of select="node()"/>
  </xsl:template>

  
  <!-- Puzzle List -->
  
  <xsl:template match="waves">
    <xsl:apply-templates select="*"/>
  </xsl:template>
  
  <xsl:template match="wave">
    <p>
      <h3><xsl:value-of select="@name"/>:</h3>
      <ul class="puzzle-list"><xsl:apply-templates select="*"/></ul>
    </p>
  </xsl:template>
  
  <xsl:template match="puzzle">
    <li>
      <a href="puzzle/{@key}"><xsl:value-of select="@name"/><xsl:apply-templates select="*"/></a>
    </li>
  </xsl:template>

  <xsl:template match="hint">
    <span class="spacing"/>
    <a href="hint/{@key}.xml">[hint <xsl:value-of select="@number"/>]</a>
  </xsl:template>
  
  <xsl:template match="prose">
    <xsl:copy-of select="*"/>
  </xsl:template>


  <!-- Forms -->

  <xsl:template match="form">
    <form method="post">
      <xsl:apply-templates select="*"/>
    </form>
  </xsl:template>

  <xsl:template match="section">
    <p>
      <table>
        <tbody>
          <xsl:for-each select="input">
            <tr>
              <xsl:call-template name="input"/>
            </tr>
          </xsl:for-each>
        </tbody>
      </table>
    </p>
  </xsl:template>

  <xsl:template name="input">
    <xsl:if test="contains(@type, 'password')">
      <td><xsl:value-of select="normalize-space(.)"/>:</td>
      <td><input type="password"
      name="{@id}" value="{@value}"
      class="form-cell text"/></td>
    </xsl:if>
    <xsl:if test="contains(@type, 'text')">
      <td><xsl:value-of select="normalize-space(.)"/>:</td>
      <td><input type="text"
      name="{@id}" value="{@value}"
      class="form-cell text"/></td>
    </xsl:if>
    <xsl:if test="contains(@type, 'number')">
      <td><xsl:value-of select="normalize-space(.)"/>:</td>
      <td><input type="text"
      name="{@id}" value="{@value}"
      class="form-cell number"/></td>
    </xsl:if>
    <xsl:if test="contains(@type, 'fixed')">
      <td><xsl:value-of select="normalize-space(.)"/>:</td>
      <td><input type="text"
      readonly="true"
      name="{@id}" value="{@value}"
      class="form-cell text"/></td>
    </xsl:if>
    <xsl:if test="contains(@type, 'boolean')">
      <td><xsl:value-of select="normalize-space(.)"/>:</td>
      <xsl:if test="@value='true'">
        <td><input type="checkbox"
        name="{@id}" checked=""
        class="form-cell checkbox"/></td>
      </xsl:if>
      <xsl:if test="not(@value='true')">
        <td><input type="checkbox"
        name="{@id}"
        class="form-cell checkbox"/></td>
      </xsl:if>
    </xsl:if>
    <xsl:if test="contains(@type, 'puzzle')">
      <td><xsl:value-of select="normalize-space(.)"/>:</td>
      <td><xsl:call-template name="PuzzleInput"/></td>
    </xsl:if>
  </xsl:template>
  
  <xsl:template match="expandable-section">
    <p>
      <table>
        <tbody id="{@itemkey}-table" action="{@action}">
          <tr>
            <xsl:for-each select="input">
              <th><xsl:value-of select="normalize-space(@name)"/></th>
            </xsl:for-each>
          </tr>
          <tr id="{@itemkey}-template" style="display:none">
            <xsl:for-each select="input">
              <td>
                <xsl:call-template name="expandable-input"/>
              </td>
            </xsl:for-each>
            <td>
              <a href="#" onclick="deleteRow(this)" class="delete-mark">✘</a>
            </td>
          </tr>
        </tbody>
      </table>
      <a href="#" onclick="addRow('{@hunt}', '{@itemkey}')">
        Add <xsl:value-of select="@item"/>
      </a>
    </p>
    <script type="text/javascript">
      onLoad(function() {
      var item = '<xsl:value-of select="@itemkey"/>';
      var hunt = '<xsl:value-of select="@hunt"/>';
      <xsl:for-each select="row">
        var row = [<xsl:for-each select="value">'<xsl:value-of select="normalize-space(.)"/>', </xsl:for-each>];
        insertRow(hunt, item, row);
      </xsl:for-each>
      });
    </script>
  </xsl:template>

  <xsl:template name="expandable-input">
    <xsl:if test="contains(@type, 'mini')">
      <input type="text"
             name="{@id}" value="{@value}"
             class="multi-form-cell mini"/>
    </xsl:if>
    <xsl:if test="contains(@type, 'number')">
      <input type="text"
             name="{@id}" value="{@value}"
             class="multi-form-cell number"/>
    </xsl:if>
    <xsl:if test="contains(@type, 'long')">
      <input type="text"
             name="{@id}" value="{@value}"
             class="multi-form-cell long"/>
    </xsl:if>
    <xsl:if test="contains(@type, 'text')">
      <input type="text"
             name="{@id}" value="{@value}"
             class="multi-form-cell text"/>
    </xsl:if>
    <xsl:if test="contains(@type, 'fixed')">
      <input type="text"
             name="{@id}" value="{@value}"
             readonly="true"
             class="multi-form-cell fixed-text"/>
    </xsl:if>
    <xsl:if test="contains(@type, 'datetime')">
      <input type="text"
             name="{@id}" value="{@value}"
             class="multi-form-cell datetime"
             step="1"/>
    </xsl:if>
    <xsl:if test="contains(@type, 'boolean')">
      <xsl:if test="@value='true'">
        <input type="checkbox"
               name="{@id}" checked=""
               class="multi-form-cell checkbox"/>
      </xsl:if>
      <xsl:if test="not(@value='true')">
        <input type="checkbox"
               name="{@id}"
               class="multi-form-cell checkbox"/>
      </xsl:if>
    </xsl:if>
    <xsl:if test="contains(@type, 'puzzle')">
      <xsl:call-template name="PuzzleInput"/>
    </xsl:if>
    <xsl:if test="contains(@type, 'wave')">
      <xsl:call-template name="WaveInput"/>
    </xsl:if>
  </xsl:template>

  <xsl:template match="submit-button">
    <input type="submit"
           value="{normalize-space(.)}"/>
    <br/>
  </xsl:template>

  
  <!-- Page Template -->

  <xsl:template match="page">
    <xsl:variable name="hunt">
      <xsl:value-of select="hunt"/>
    </xsl:variable>
    <html>
      <head>
        <title><xsl:value-of select="hunt"/></title>
        <link rel="stylesheet" type="text/css" href="/css/style.css"/>
        <script type="text/javascript" src="/ph.js"/>
      </head>
      <body>
        <ul class="nav">
          <xsl:if test="not(@page-type)">
            <li class="nav1">
              <a href="index.xml"><xsl:value-of select="hunt"/></a>
            </li>
            <li class="nav2"><a href="team.xml">Team</a></li>
            <li class="nav3"><a href="leaderboard.xml">Leaderboard</a></li>
            <li class="nav4"><a href="puzzle-stats.xml">Puzzle Stats</a></li>
            <li class="nav5"><a href="puzzles.xml">Puzzles</a></li>
          </xsl:if>
          <xsl:if test="@page-type = 'admin'">
            <li class="nav1">
              <a href="edit-hunt.xml"><xsl:value-of select="hunt"/></a>
            </li>
            <li class="nav2"><a href="edit-waves.xml">Waves</a></li>
            <li class="nav3"><a href="edit-puzzles.xml">Puzzles</a></li>
            <li class="nav4"><a href="edit-hints.xml">Hints</a></li>
            <li class="nav5"><a href="view-teams.xml">Teams</a></li>
            <li class="nav6"><a href="signout.xml">Sign Out</a></li>
          </xsl:if>
          <xsl:if test="@page-type = 'global'">
            <li class="nav-global"><a href="/">PuzzleHunt: PH</a></li>
          </xsl:if>
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

