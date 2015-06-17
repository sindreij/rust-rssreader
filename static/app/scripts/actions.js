var Constants = require('./constants');

module.exports = {
    loadFeeds: function() {
        this.dispatch(Constants.LOAD_FEEDS);

        $.get('/feed')
            .done(function(data) {
                this.dispatch(Constants.LOAD_FEEDS_SUCCESS, {feeds: data});
            }.bind(this))
            .fail(function(error) {
                this.dispatch(Constants.LOAD_FEEDS_FAIL, {error: error});
            }.bind(this));
    },

    selectFeed: function(feed_id) {
        this.dispatch(Constants.LOAD_POSTS, {'id': feed_id});
        $.get('/feed/' + feed_id + '/posts')
            .done(function(data) {
                this.dispatch(Constants.LOAD_POSTS_SUCCESS, {posts: data});
            }.bind(this))
            .fail(function(error) {
                this.dispatch(Constants.LOAD_POSTS_FAIL, {error: error});
            }.bind(this));
    },
}
