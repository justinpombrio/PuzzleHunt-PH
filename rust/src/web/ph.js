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

function addRow(id) {
  var rowTemplate = get(id + "-template");
  var row = rowTemplate.cloneNode(true);
  row.style.display = "";
  delete row.id;
  row.name = "a-row";
/*  for (var i = 0; i < row.children.length; i++) {
    var child = row.children[i];
    if (!child.children || child.children.length === 0) { continue; }
    var cell = child.children[0];
    if (cell.name === "key") {
      var box = child.removeChild(cell);
      var filename = data ? data["key"] : randomFilename();
      box.value = filename;
      var link = make("a", {
        "href": "/" + item + "/" + filename + ".xml",
        "value": box.value
      });
      link.appendChild(box);
      child.appendChild(link);
    } else if (cell.name === "time") {
      if (data && data.hasOwnProperty(cell.name)) {
        var local_time = new Date(data[cell.name]);
        cell.value = local_time.toLocaleString();
      }
    } else if (data && data.hasOwnProperty(cell.name)) {
      cell.value = data[cell.name];
    }
  } */
  var table = get(id + "-table");
  table.appendChild(row);
}

window.onload = function() {
  for (i in ON_LOAD) {
    ON_LOAD[i]();
  }
}
