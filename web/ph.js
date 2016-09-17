
/* DOM Utilities */

function error(msg) {
  alert(msg);
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

function setupMultiForm() {
  if (get("multi-form")) {
    addRow();
  }
}

window.onload = function() {
  setupMultiForm();
  setupPuzzleInput();
}

/* Form submission */

function getInputs() {
  var inputs = getTags("input").concat(getTags("select"));
  var dict = {};
  for (var i = 0; i < inputs.length; i++) {
    var input = inputs[i];
    dict[input.name] = "" + input.value;
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
  if (action === "register") {
    var pass1 = inputs["password"];
    var pass2 = inputs["password_verify"];
    if (pass1 !== pass2) {
      error("Passwords do not match.")
    }
    delete inputs["password_verify"];
  }
  var json = JSON.stringify(getInputs());
  console.log("POST", "/" + action, json);
}

function submitMultiForm(action, item) {
  var dict = {}; dict[item] = getMultiInputs(item);
  var json = JSON.stringify(dict);
  console.log("POST", "/" + action, json);
}
