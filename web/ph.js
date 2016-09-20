
var SERVER_ADDRESS = "http://52.38.39.79:4000/";



/* DOM Utilities */

function success(msg) {
  console.log("SUCCESS", msg);
  get("failure-message").textContent = "";
  get("success-message").textContent = msg;
}

function failure(msg) {
  console.log("ERROR", msg);
  if (msg === "Unauthorized") {
//    window.location.href = "/master/login.xml";
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



/* Other Utilities */

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

  try {
    var response = JSON.parse(request.response);
    console.log("RESPONSE", response, request.getAllResponseHeaders());
    switch (response.status) {
    case "Failure":
      failure(response.message);      
      break;
    case "Success":
      success("Successfully updated");
      if (onSuccess !== undefined) {
        return onSuccess(response);
      } else {
        return response;
      }
    }
  } catch (exn) {
    panic("Invalid response", exn);
  }
  return null;
}



/* Query Strings */

function parseQuery(qstr) {
  var query = {};
  var a = qstr.substr(1).split('&');
  for (var i = 0; i < a.length; i++) {
    var b = a[i].split('=');
    query[decodeURIComponent(b[0])] = decodeURIComponent(b[1] || '');
  }
  return query;
}

var QUERY = parseQuery(window.location.search);



/* List Puzzles */

function getPuzzles() {
  return post("viewPuzzles", {}, function(response) {
    return response.puzzles;
  });
}

//var PUZZLES = getPuzzles();


/* Forms */

function setupPuzzleInput() {
  var dropdown = get("puzzle-input");
  if (dropdown) {
    console.log("PUZZLES", PUZZLES);
    for (var i = 0; i < PUZZLES.length; i++) {
      var puzzle = PUZZLES[i];
      var option = make('option');
      option.text = i;
      option.value = i;
      dropdown.add(option, 0);
    }
  }
  
  function setDropdownOption(dropdown, value) {
    var opts = dropdown.options;
    for (var i = 0; i < opts.length; i++) {
      if (opts[i].value === value) { opts[i].selected = "selected"; }
    }
  }
  var dropdown = get("puzzle-input");
  if (dropdown) {
    setDropdownOption(dropdown, QUERY['puzzle']);
  }
}

function deleteNode(node) {
  node.parentNode.removeChild(node);
}

function deleteRow(self) {
  deleteNode(self.parentNode.parentNode);
}

function addRow() {
  var rowTemplate = get("row-template");
  var row = rowTemplate.cloneNode(true);
  row.style.display = "";
  for (var i = 0; i < row.children.length; i++) {
    var child = row.children[i];
    if (!child.children || child.children.length === 0) { continue; }
    var cell = child.children[0];
    if (cell.name === "key") {
      cell.value = randomFilename();
    }
  }
  var form = get("multi-form");
  form.appendChild(row);
}

function setupForm() {
  var form = get("form");
  if (form) {
    var action = form.getAttribute("action");
    if (action) {
      performAction(action);
    }
  }
  
  if (!QUERY.data) { return; }
  var json = JSON.parse(QUERY.data);
  switch (json.location) {
  case "your-team":
    get("name").value = json.name
    get("guesses").value = json.guesses
    for (var i = 0; i < json.members.length; i++) {
      get("member_name_" + (i+1)).value = json.members[i]["name"];
      get("member_email_" + (i+1)).value = json.members[i]["email"];
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
  if (get("multi-form")) {
    addRow();
  }
}

window.onload = function() {
  setupForm();
  setupMultiForm();
  setupPuzzleInput();
}



/* Form submission */

function getInputs() {
  var inputs = getTags("input").concat(getTags("select"));
  var dict = {};
  for (var i = 0; i < inputs.length; i++) {
    var input = inputs[i];
    dict[input.id] = "" + input.value;
  }
  return dict;
}

function getMultiInputs() {
  var rows = [];
  var form = get("multi-form");
  for (var i = 2; i < form.children.length; i++) {
    var row = form.children[i];
    var dict = {};
    for (var j = 1; j < row.children.length; j++) {
      var input = row.children[j].children[0];
      dict[input.name] = input.value;
    }
    rows.push(dict);
  }
  var dict = {};
  dict[item] = rows;
  return dict;
}



/* Form actions */

function performAction(action) {
  function goTo(location, response) {
    response.location = location;
    window.location.href = location + ".xml?data=" + JSON.stringify(response);
  }
  switch (action) {

  case "login":
    var inputs = getInputs();
    return post("login", inputs, function(response) {
      console.log("LOGGED IN?");
//      goTo("hunt", response);
    });
    
  case "viewOwnTeam":
    var inputs = getInputs();
    return post("viewOwnTeam", inputs, function(response) {
      goTo("your-team", response);
    });

  case "setHunt":
    var inputs = getInputs();
    return post("setHunt", inputs);

  case "getHunt":
    return post("getHunt", {}, function(response) {
      get("name").value = response.name;
      get("team-size").value = response.teamSize;
      get("init-guesses").value = response.initGuesses;
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
    return post("registerTeam", inputs);
    
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
    return post("changeMembers", inputs);
  }
}

function submitMultiForm(action, item) {
  post(action, dict);
}
