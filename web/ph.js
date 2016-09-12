function parseQuery(qstr) {
  var query = {};
  var a = qstr.substr(1).split('&');
  for (var i = 0; i < a.length; i++) {
    var b = a[i].split('=');
    query[decodeURIComponent(b[0])] = decodeURIComponent(b[1] || '');
  }
  return query;
}

function setupDropdown() {
  function setDropdownOption(dropdown, value) {
    var opts = dropdown.options;
    for (var i = 0; i < opts.length; i++) {
      if (opts[i].value === value) { opts[i].selected = "selected"; }
    }
  }
  document.addEventListener("DOMContentLoaded", function() {
    var dropdown = document.getElementById("puzzle-input");
    setDropdownOption(dropdown, QUERY['puzzle']);
  });
}

function deleteNode(node) {
  node.parentNode.removeChild(node);
}

function deleteRow(self) {
  deleteNode(self.parentNode.parentNode);
}

function addRow() {
  var rowTemplate = document.getElementById("row-template");
  var row = rowTemplate.cloneNode(true);
  row.style.display = "";
  row.id = "";
  var form = document.getElementById("multi-form-rows");
  form.appendChild(row);
}

function setupMultiform() {
  document.addEventListener("DOMContentLoaded", function() {
    addRow();
  });
}

var QUERY = parseQuery(window.location.search);
