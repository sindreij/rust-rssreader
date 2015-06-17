var React = require("react"),
    Fluxxor = require("fluxxor"),
    FluxMixin = Fluxxor.FluxMixin(React),
    StoreWatchMixin = Fluxxor.StoreWatchMixin;

var FeedList = require('./feed_list'),
    PostList = require('./post_list');

var Application = React.createClass({
    mixins: [FluxMixin, StoreWatchMixin("FeedStore")],

    getStateFromFlux: function() {
        var flux = this.getFlux();
        return {
            feeds: flux.store('FeedStore').getState()
        };
    },

    render: function() {
        return (
            <div className="app">
                <FeedList
                    feeds={this.state.feeds.feeds}
                    onSelectFeed={this.onSelectFeed} />
                <PostList
                    posts={this.state.feeds.current_posts} />
            </div>
        )
    },
    componentDidMount: function() {
        this.getFlux().actions.loadFeeds();
    },
    onSelectFeed: function(feed_id) {
        this.getFlux().actions.selectFeed(feed_id);
    }
});

module.exports = Application;
