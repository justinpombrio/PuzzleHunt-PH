"use strict";

var ON_LOAD = [];

function onLoad(f) {
  ON_LOAD.push(f);
}

function get(id) {
  return document.getElementById(id);
}

function deleteNode(node) {
  node.parentNode.removeChild(node);
}

function deleteRow(self) {
  deleteNode(self.parentNode.parentNode);
}

function makeRow(id) {
  var rowTemplate = get(id + "-template");
  var row = rowTemplate.cloneNode(true);
  row.style.display = "";
  delete row.id;
  row.name = "a-row";
  return row;
}

function addRow(hunt, id) {
  console.log("@addRow", hunt);
  var row = makeRow(id);
  for (var i = 0; i < row.children.length; i++) {
    var child = row.children[i];
    if (!child.children || child.children.length === 0) { continue; }
    var cell = child.children[0];
    if (cell.name === "key") {
      var box = child.removeChild(cell);
      let filename = randomFilename(6) + (id === "puzzle" ? ".pdf" : ".xml");
      child.appendChild(makeKeyLink(hunt, id, box, filename));
    }/* else if (cell.name === "datetime") {
      if (data && data.hasOwnProperty(cell.name)) {
        var local_time = new Date(data[cell.name]);
        cell.value = local_time.toLocaleString();
      }
    } else if (data && data.hasOwnProperty(cell.name)) {
      cell.value = data[cell.name];
    }*/
  }
  var table = get(id + "-table");
  table.appendChild(row);
}

function makeKeyLink(hunt, id, box, filename) {
  box.value = filename;
  var link = make("a", {
    "href": "/" + hunt + "/" + id + "/" + filename,
    "value": box.value
  });
  link.appendChild(box);
  return link;
}

function insertRow(hunt, id, values) {
  console.log("@insertRow", hunt);
  var row = makeRow(id);
  for (var i in values) {
    var value = values[i];
    var child = row.cells[i];
    var cell = child.childNodes[0];
    if (cell.name === "key") {
      var box = child.removeChild(cell);
      child.appendChild(makeKeyLink(hunt, id, box, value));
    } else {
      cell.value = value;
    }
  }
  var table = get(id + "-table");
  table.appendChild(row);
}

function randomFilename(len) {
  var text = "";
  var possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  for (var i = 0; i < len; i++) {
    let n = Math.floor(Math.random() * possible.length);
    text += possible[n];
  }
  return text;
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

window.onload = function() {
  for (var i in ON_LOAD) {
    ON_LOAD[i]();
  }
}
