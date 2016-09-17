
var SERVER_ADDRESS = "http://52.38.39.79:4000/";

/* DOM Utilities */

function error(msg) {
  console.log("ERROR");
  alert(msg);
}

function panic(msg) {
  error("Oops! The site broke. Details logged to console.");
  console.log("INTERNAL ERROR");
  console.log(msg);
}

function get(id) {
  return document.getElementById(id);
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

// taken from http://stackoverflow.com/questions/133925/javascript-post-request-like-a-form-submit#133997
function post(action, params) {
  var path = SERVER_ADDRESS + action;
  var json = JSON.stringify(params);
  console.log("POST", path, json);
  
  var request = new XMLHttpRequest();
  request.open("POST", path, false);
  request.setRequestHeader("Content-Type", "application/json; charset=UTF-8");
  request.send(json);

  try {
    var response = JSON.parse(request.response);
    switch (response.status) {
    case "Failure":
      error(response.message);      
      break;
    case "Success":
      return response;
    }
  } catch (exn) {
    console.log(exn);
    panic("Invalid response");
  }
  return null;
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

/* Forms */

function setupPuzzleInput() {
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
  renumberForm();
}

function addRow() {
  var rowTemplate = get("row-template");
  var row = rowTemplate.cloneNode(true);
  row.style.display = "";
  row.id = "";
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
  renumberForm();
}

function renumberForm() {
  var form = get("multi-form");
  for (var i = 2; i < form.children.length; i++) {
    var child = form.children[i];
    child.children[0].textContent = i - 2;
  }
}

function setupForm() {
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
  return rows;
}

function submitForm(action) {
  var inputs = getInputs();
  switch (action) {
  case "registerTeam":
    var pass1 = inputs["password"];
    var pass2 = inputs["password_verify"];
    if (pass1 !== pass2) {
      error("Passwords do not match.")
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
    break;
  case "changeMembers":
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
    break;
  }
  var response = post(action, inputs);
  if (response !== null) {
    handleResponse(action, response);
  }
}

function submitMultiForm(action, item) {
  var dict = {}; dict[item] = getMultiInputs(item);
  post(action, dict);
}

/* Form actions */

function handleResponse(action, response) {
  console.log("SUCCESS", response);
  function goto(location) {
    response.location = location;
    window.location.href = location + ".xml?data=" + JSON.stringify(response);
  }
  switch (action) {
  case "registerTeam":
    break;
  case "viewOwnTeam":
    goto("your-team");
    break;
  case "changeMembers":
    break;
  }
}
