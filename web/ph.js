function parseQuery(qstr) {
  var query = {};
  var a = qstr.substr(1).split('&');
  for (var i = 0; i < a.length; i++) {
    var b = a[i].split('=');
    query[decodeURIComponent(b[0])] = decodeURIComponent(b[1] || '');
  }
  return query;
}

function setDropdownOption(dropdown, value) {
  var opts = dropdown.options;
  for (var i = 0; i < opts.length; i++) {
    if (opts[i].value === value) { opts[i].selected = "selected"; }
  }
}

var QUERY = parseQuery(window.location.search);
