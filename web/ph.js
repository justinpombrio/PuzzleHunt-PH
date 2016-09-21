
var SERVER_ADDRESS = "http://52.38.39.79:4000/";

window.onload = setup;



/******************** DOM Utilities ********************/

function success(msg) {
  console.log("SUCCESS", msg);
  get("failure-message").textContent = "";
  get("success-message").textContent = msg;
}

function failure(msg) {
  console.log("ERROR", msg);
  if (msg === "Unauthorized") {
    window.location.href = "/login.xml";
  }
  get("success-message").textContent = "";
  get("failure-message").textContent = msg;
}

function panic(msg, details) {
  failure("Oops! The site broke. Details logged to console.");
  console.log("INTERNAL ERROR", msg);
  console.log(details);
}

function get(id) {
  return document.getElementById(id);
}

function getByName(name) {
  return document.getElementsByName(name);
}

function getByClass(name) {
  return document.getElementsByClassName(name);
}

function make(nodeType, attrs) {
  var elem = document.createElement(nodeType);
  if (attrs !== undefined) {
    for (var attr in attrs) {
      elem[attr] = attrs[attr];
    }
  }
  return elem;
}

function makeText(text) {
  return document.createTextNode(text);
}

function getTags(tagName) {
  var ickyTags = document.getElementsByTagName(tagName);
  var tags = [];
  for (var i = 0; i < ickyTags.length; i++) {
    var tag = ickyTags[i];
    if (tag.type === "button") { continue; }
    tags.push(tag);
  }
  return tags;
}

// Taken from http://stackoverflow.com/questions/5898656/test-if-an-element-contains-a-class#5898748
function hasClass(element, cls) {
  return (' ' + element.className + ' ').indexOf(' ' + cls + ' ') > -1;
}


/******************** Other Utilities ********************/

var SAFE_CHARS = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

function randomFilename() {
  var filename = "";
  for (var i = 0; i < 10; i++) {
    var index = Math.floor(SAFE_CHARS.length * Math.random());
    filename += SAFE_CHARS[index];
  }
  return filename;
}

function secondsToHours(secs) {
  return Math.round( secs / 3600 * 10) / 10;
}


/* Http POST */

// taken from http://stackoverflow.com/questions/133925/javascript-post-request-like-a-form-submit#133997
function post(action, params, onSuccess) {
  var path = SERVER_ADDRESS + action;
  var json = JSON.stringify(params);
  console.log("POST", path, json);
  
  var request = new XMLHttpRequest();
  request.open("POST", path, false);
  request.setRequestHeader("Content-Type", "application/json; charset=UTF-8");
  request.send(json);

//  try {
    var response = JSON.parse(request.response);
    console.log("RESPONSE", response, request.getAllResponseHeaders());
    switch (response.status) {
    case "Failure":
      failure(response.message);      
      break;
    case "Success":
      if (onSuccess !== undefined) {
        return onSuccess(response);
      } else {
        return response;
      }
    }
//  } catch (exn) {
//    panic("Invalid response", exn);
//  }
  return null;
}

/* Query Strings */

var QUERY_CACHE = null;

function getQuery() {
  if (QUERY_CACHE === null) {
    QUERY_CACHE = parseQuery(window.location.search);
  }
  return QUERY_CACHE;
}

function parseQuery(qstr) {
  var query = {};
  var a = qstr.substr(1).split('&');
  for (var i = 0; i < a.length; i++) {
    var b = a[i].split('=');
    query[decodeURIComponent(b[0])] = decodeURIComponent(b[1] || '');
  }
  return query;
}



/******************** Puzzle/Wave Lists ********************/

var PUZZLES_CACHE = null;
var WAVES_CACHE = null;
var HUNT_CACHE = null;

function getPuzzles() {
  if (PUZZLES_CACHE === null) {
    PUZZLES_CACHE = post("viewPuzzles", {}, function(response) {
      return response.puzzles;
    });
  }
  return PUZZLES_CACHE;
}

function getAllPuzzles() {
  if (PUZZLES_CACHE === null) {
    PUZZLES_CACHE = post("getPuzzles", {}, function(response) {
      return response.puzzles;
    });
  }
  return PUZZLES_CACHE;
}

function getWaves() {
  if (WAVES_CACHE === null) {
    WAVES_CACHE = post("getWaves", {}, function(response) {
      return response.waves;
    });
  }
  return WAVES_CACHE;
}

function getHunt() {
  if (HUNT_CACHE === null) {
    HUNT_CACHE = post("viewHunt", {}, function(response) {
      return response;
    });
  }
  return HUNT_CACHE;
}



/******************** Form Utilities ********************/

function deleteNode(node) {
  node.parentNode.removeChild(node);
}

function deleteRow(self) {
  deleteNode(self.parentNode.parentNode);
}

function deleteRows() {
  var rows = getByName("a-row");
  for (var i = 0; i < rows.length; i++) {
    deleteRow(rows[i]);
  }
}

function clearTable() {
  var rows = getByName("a-table-row");
  for (var i = 0; i < rows.length; i++) {
    deleteRow(rows[i]);
  }
}

function addRow(data, item) {
  var rowTemplate = get("row-template");
  var row = rowTemplate.cloneNode(true);
  row.style.display = "";
  delete row.id;
  row.name = "a-row";
  for (var i = 0; i < row.children.length; i++) {
    var child = row.children[i];
    if (!child.children || child.children.length === 0) { continue; }
    var cell = child.children[0];
    if (cell.name === "key") {
      var box = child.removeChild(cell);
      var filename = data ? data["key"] : randomFilename();
      box.value = filename;
      var link = make("a", {
        "title": filename,
        "href": "/" + item + "/" + filename + ".xml",
        "value": box.value
      });
      link.appendChild(box);
      child.appendChild(link);
    }
    if (data && data.hasOwnProperty(cell.name)) {
      cell.value = data[cell.name];
    }
  }
  get("multi-form").appendChild(row);
}

function addTableRow(data) {
  var cols = [];
  var headers = get("table").children[0].children;
  for (var i = 0; i < headers.length; i++) {
    cols.push(headers[i].id);
  }
  var tr = make("tr", { "name": "a-table-row" });
  for (var i = 0; i < cols.length; i++) {
    var col = cols[i];
    tr.appendChild(make("td", {
      "textContent": data[col],
      "className": "table-cell"
    }));
  }
  get("table").appendChild(tr);
}



/******************** Page Setup ********************/

function setupHunt() {
  var hunt = getHunt();
  // Set page title
  document.title = hunt.name;
  var title = get("hunt-title");
  if (title) { title.textContent = hunt.name; }
}

function setupPuzzleList() {
  var list = get("all-puzzles");
  if (!list) { return; }
  var puzzles = getPuzzles();

  // Gather waves
  var waves = [];
  for (var i = 0; i < puzzles.length; i++) {
    var puzzle = puzzles[i];
    var wave = puzzle.wave;
    if (waves.indexOf(wave) == -1) {
      waves.push(wave);
    }
  }

  // List the puzzles by wave
  for (var i = 0; i < waves.length; i++) {
    var wave = waves[i];
    var sublist = make("ul");
    sublist.classList.add("puzzle-list");
    for (var j = 0; j < puzzles.length; j++) {
      var puzzle = puzzles[j];
      if (puzzle.wave === wave) {
        var link = make("a", {
          "textContent": puzzle.name,
          "title": puzzle.name,
          "href": "puzzles/" + puzzle.key + ".xml"
        });
        var elem = make("li");
        elem.appendChild(link);
        for (var k = 0; k < puzzle.hints.length; k++) {
          var hint = puzzle.hints[k];
          elem.appendChild(make("span", {"className": "spacing"}));
          elem.appendChild(make("a", {
            "textContent": "Hint " + hint.number,
            "title":       "Hint " + hint.number,
            "href":        "hints/" + hint.key + ".xml"
          }));
        }
        sublist.appendChild(elem);
      }
    }
    var waveli = make("p", {
      "textContent": wave + ":"
    });
    waveli.appendChild(sublist);
    list.appendChild(waveli);
  }
}

function setupForm() {
  var form = get("form");
  if (form) {
    var action = form.getAttribute("action");
    if (action) {
      performAction(action);
    }
  }
  
  if (!getQuery().data) { return; }
  var json = JSON.parse(getQuery().data);
  switch (json.location) {
  case "/your-team.xml":
    getByName("name")[0].value = json.name
    getByName("guesses")[0].value = json.guesses
    for (var i = 0; i < json.members.length; i++) {
      getByName("member_name_" + (i+1))[0].value = json.members[i]["name"];
      getByName("member_email_" + (i+1))[0].value = json.members[i]["email"];
    }
    break;
  }
}

function setupMultiForm() {
  var form = get("multi-form");
  if (form) {
    var action = form.getAttribute("action");
    if (action) {
      performAction(action);
    }
  }
}

function setupInput(dropdowns, choices, selection) {
  if (dropdowns && dropdowns.length > 0) {
    for (var i = 0; i < choices().length; i++) {
      var choice = choices()[i];
      for (var j = 0; j < dropdowns.length; j++) {
        var selected = choice.name === selection ? "selected" : undefined;
        var option = make('option', {
          "text": choice.name,
          "value": choice.name,
          "selected": selected
        });
        var dropdown = dropdowns[j];
        dropdown.add(option, 0);
      }
    }
  }
}

function setupPuzzleInputs() {
  var get = window.location.pathname.indexOf("/master/") === -1
    ? getPuzzles : getAllPuzzles;
  setupInput(getByName("puzzle"), get, getQuery()['puzzle']);
}

function setupWaveInputs() {
  setupInput(getByName("wave"), getWaves);
}

function setup() {
  setupHunt();
  setupPuzzleInputs(); 
  setupWaveInputs();
  setupForm();
  setupMultiForm();
  setupPuzzleList();
}




/******************** Form Submission ********************/

function getInput(dict, input) {
  if (input.tagName.toLowerCase() === "a" // duct tape
      && input.children
      && input.children[0]) {
    console.log("!", input.children[0]);
    input = input.children[0];
  }
  if (hasClass(input, "number")) {
    dict[input.name] = parseInt(input.value);
  } else {
    dict[input.name] = input.value;
  }
}

function getInputs() {
  var inputs = getTags("input").concat(getTags("select"));
  var dict = {};
  for (var i = 0; i < inputs.length; i++) {
    var input = inputs[i];
    getInput(dict, input);
  }
  return dict;
}

function getMultiInputs(item) {
  var rows = [];
  var form = get("multi-form");
  for (var i = 2; i < form.children.length; i++) {
    var row = form.children[i];
    var dict = {};
    for (var j = 0; j < row.children.length; j++) {
      var input = row.children[j].children[0];
      getInput(dict, input);
    }
    rows.push(dict);
  }
  var dict = {};
  dict[item] = rows;
  return dict;
}



/******************** Filling Out Forms ********************/

function fillForm(data) {
  var cells = getByClass("form-cell");
  for (var i = 0; i < cells.length; i++) {
    cells[i].value = data[cells[i].name];
  }
}

function fillMultiForm(datas, item) {
  deleteRows();
  for (var i = 0; i < datas.length; i++) {
    var data = datas[i];
    addRow(data, item);
  }
}

function fillTable(datas) {
  clearTable();
  for (var i = 0; i < datas.length; i++) {
    var data = datas[i];
    addTableRow(data);
  }
}



/******************** Form Actions ********************/

function performAction(action) {

  function goTo(location, response) {
    if (response) {
      response.location = location;
      window.location.href = location + "?data=" + JSON.stringify(response);
    } else {
      window.location.href = location;
    }
  }

  switch (action) {
    
  /* Master Actions */

  case "login":
    var inputs = getInputs();
    return post("login", inputs, function(response) {
      goTo("/master/hunt.xml", response);
    });

  case "logout":
    return post("logout", {}, function() {
      goTo("/index.xml");
    });

  case "getHunt":
    return post("getHunt", {}, function(response) {
      fillForm(response);
    });

  case "getPuzzles":
    return post("getPuzzles", {}, function(response) {
      fillMultiForm(response.puzzles, "puzzles");
    });

  case "getWaves":
    return post("getWaves", {}, function(response) {
      fillMultiForm(response.waves);
    });

  case "getHints":
    return post("getHints", {}, function(response) {
      fillMultiForm(response.hints, "hints");
    });

  case "setHunt":
    var inputs = getInputs();
    return post("setHunt", inputs, function() {
      success("Successfully updated.");
    });

  case "setPuzzles":
    var inputs = getMultiInputs("puzzles");
    return post("setPuzzles", inputs, function() {
      success("Successfully updated.");
    });

  case "setWaves":
    var inputs = getMultiInputs("waves");
    return post("setWaves", inputs, function() {
      success("Successfully updated.");
    });

  case "setHints":
    var inputs = getMultiInputs("hints");
    return post("setHints", inputs, function() {
      success("Successfully updated.");
    });

  /* Puzzler Actions */

  case "viewOwnTeam":
    var inputs = getInputs();
    return post("viewOwnTeam", inputs, function(response) {
      goTo("/your-team.xml", response);
    });
    
  case "registerTeam":
    var inputs = getInputs();
    var pass1 = inputs["password"];
    var pass2 = inputs["password_verify"];
    if (pass1 !== pass2) {
      failure("Passwords do not match.");
      return;
    }
    delete inputs["password_verify"];
    var members = [];
    for (var i = 1; i <= 4; i++) {
      var name = inputs["member_name_" + i];
      var email = inputs["member_email_" + i];
      if (email !== "") {
        members.push({"name": name, "email": email});
      }
      delete inputs["member_name_" + i];
      delete inputs["member_email_" + i];
    }
    inputs["members"] = members;
    return post("registerTeam", inputs, function() {
      success("Successfully registered.");
    });
    
  case "changeMembers":
    var inputs = getInputs();
    var members = [];
    for (var i = 1; i <= 4; i++) {
      var name = inputs["member_name_" + i];
      var email = inputs["member_email_" + i];
      if (email !== "") {
        members.push({"name": name, "email": email});
      }
      delete inputs["member_name_" + i];
      delete inputs["member_email_" + i];
    }
    inputs["members"] = members;
    delete inputs["guesses"];
    return post("changeMembers", inputs, function() {
      success("Successfully updated.");
    });
    
  case "submitGuess":
    var inputs = getInputs();
    return post("submitGuess", inputs, function(response) {
      switch (response.isCorrect) {
      case "Correct":
        success("Correct!");
        break;
      case "Incorrect":
        failure("Incorrect.");
        break;
      case "OutOfGuesses":
        failure("You are out of guesses!");
        break;
      }
    });

  case "getMembers":
    return post("getMembers", {}, function(response) {
      fillTable(response.members);
    });

  case "viewTeamsStats":
    return post("viewTeamsStats", {}, function(response) {
      for (var i = 0; i < response.teams.length; i++) {
        var team = response.teams[i];
        team.avgSolveTime = secondsToHours(team.avgSolveTime);
      }
      fillTable(response.teams);
    });

  case "viewPuzzlesStats":
    return post("viewPuzzlesStats", {}, function(response) {
      for (var i = 0; i < response.puzzles.length; i++) {
        var puzzle = response.puzzles[i];
        puzzle.avgSolveTime = secondsToHours(puzzle.avgSolveTime);
      }
      fillTable(response.puzzles);
    });


  
  }
}

function submitMultiForm(action, item) {
  post(action, dict);
}
