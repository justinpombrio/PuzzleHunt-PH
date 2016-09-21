
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
    window.location.href = "/master/login.xml";
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

function make(nodeType) {
  return document.createElement(nodeType);
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


/* Http POST */

// taken from http://stackoverflow.com/questions/133925/javascript-post-request-like-a-form-submit#133997
function post(action, params, onSuccess) {
  var path = SERVER_ADDRESS + action;
  var json = JSON.stringify(params);
  console.log("POST", path, json);
  
  var request = new XMLHttpRequest();
  request.open("POST", path, false);
  request.withCredentials = true;
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

function getPuzzles() {
  if (PUZZLES_CACHE === null) {
    PUZZLES_CACHE = post("viewPuzzles", {}, function(response) {
      console.log("PUZZLES", response.puzzles);
      return response.puzzles;
    });
  }
  return PUZZLES_CACHE;
}

function getWaves() {
  if (WAVES_CACHE === null) {
    WAVES_CACHE = post("getWaves", {}, function(response) {
      console.log("WAVES", response.waves);
      return response.waves;
    });
  }
  return WAVES_CACHE;
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

function addRow(data) {
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
      cell.value = randomFilename();
    }
    if (data && data.hasOwnProperty(cell.name)) {
      cell.value = data[cell.name];
    }
  }
  get("multi-form").appendChild(row);
}



/******************** Page Setup ********************/

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
  case "your-team":
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
    console.log("SETUP INPUT", dropdowns, choices());
    for (var i = 0; i < choices().length; i++) {
      var choice = choices()[i];
      for (var j = 0; j < dropdowns.length; j++) {
        var option = make('option');
        option.text = choice.name;
        option.value = choice.name;
        if (choice.name === selection) {
          option.selected = "selected";
        }
        var dropdown = dropdowns[j];
        dropdown.add(option, 0);
      }
    }
  }
}

function setupPuzzleInputs() {
  setupInput(getByName("puzzle"), getPuzzles, getQuery()['puzzle']);
}

function setupWaveInputs() {
  setupInput(getByName("wave"), getWaves);
}

function setup() {
  setupPuzzleInputs(); 
  setupForm();
  var loc = window.location.pathname;
  console.log("LOCATION", loc);
  if (loc === "/master/puzzles.xml" || loc === "/master/hints.xml") {
    setupWaveInputs();
  }
  setupMultiForm();
}




/******************** Form Submission ********************/

function getInput(dict, input) {
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

function fillMultiForm(datas) {
  deleteRows();
  for (var i = 0; i < datas.length; i++) {
    var data = datas[i];
    addRow(data);
  }
}



/******************** Form Actions ********************/

function performAction(action) {
  function goTo(location, response) {
    response.location = location;
    window.location.href = location + ".xml?data=" + JSON.stringify(response);
  }
  switch (action) {
    
  /* Master Actions */

  case "login":
    var inputs = getInputs();
    return post("login", inputs, function(response) {
      goTo("hunt", response);
    });    

  case "getHunt":
    return post("getHunt", {}, function(response) {
      fillForm(response);
    });

  case "getPuzzles":
    return post("getPuzzles", {}, function(response) {
      deleteRows();
      for (var i = 0; i < response.puzzles.length; i++) {
        var puzzle = response.puzzles[i];
        addRow(puzzle);
      }
    });

  case "getWaves":
    return post("getWaves", {}, function(response) {
      console.log("GETWAVES", response.waves);
      deleteRows();
      for (var i = 0; i < response.waves.length; i++) {
        var wave = response.waves[i];
        addRow(wave);
      }
    });

  case "getHints":
    return post("getHints", {}, function(response) {
      deleteRows();
      for (var i = 0; i < response.hints.length; i++) {
        var hint = response.hints[i];
        addRow(hint);
      }
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
      goTo("your-team", response);
    });
    
  case "registerTeam":
    var inputs = getInputs();
    var pass1 = inputs["password"];
    var pass2 = inputs["password_verify"];
    if (pass1 !== pass2) {
      failure("Passwords do not match.")
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
  }
}

function submitMultiForm(action, item) {
  post(action, dict);
}
