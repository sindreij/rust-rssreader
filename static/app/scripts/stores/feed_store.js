var Fluxxor = require('fluxxor'),
    Constants = require('../constants');

var FeedStore = Fluxxor.createStore({
    initialize: function(options) {
        this.feeds = [];
        this.current_posts = [];

        this.bindActions(
            Constants.LOAD_FEEDS_SUCCESS, this.onLoadFeedsSuccess,
            Constants.LOAD_POSTS_SUCCESS, this.onLoadPostsSuccess
        );
    },

    getState: function() {
        return {
            feeds: this.feeds,
            current_posts: this.current_posts,
        }
    },

    onLoadFeedsSuccess: function(payload) {
        this.feeds = payload.feeds;
        this.emit('change');
    },

    onLoadPostsSuccess: function(payload) {
        this.current_posts = payload.posts;
        this.emit('change');
    }
});

module.exports = FeedStore;
