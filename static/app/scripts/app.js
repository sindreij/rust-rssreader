'use strict';

var React = window.React = require('react'),
    Fluxxor = require('fluxxor');

var Application = require('./components/application'),
    FeedStore = require('./stores/feed_store'),
    actions = require('./actions');

var stores = {
    FeedStore: new FeedStore()
};


var flux = new Fluxxor.Flux(stores, actions);
window.flux = flux;

flux.on("dispatch", function(type, payload) {
  if (console && console.log) {
    console.log("[Dispatch]", type, payload);
  }
});

React.render(<Application flux={flux} />, document.getElementById('wrapper'));
