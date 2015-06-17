var React = require("react"),
    Fluxxor = require("fluxxor"),
    FluxMixin = Fluxxor.FluxMixin(React),
    StoreWatchMixin = Fluxxor.StoreWatchMixin;

var FeedList = React.createClass({
    mixins: [FluxMixin],

    propTypes: {
        feeds: React.PropTypes.array.isRequired,
        onSelectFeed: React.PropTypes.func.isRequired
    },
    render: function() {
        var items = this.props.feeds.map(function(feed) {
            return (
                <li key={feed.id}>
                    <a href="" onClick={this.onClickFeed.bind(this, feed.id)}>
                        {feed.title}
                    </a>
                </li>
            );
        }.bind(this));
        return (
            <div className="feeds">
                <h5>Feeds</h5>
                <ul className="side-nav">
                    {items}
                </ul>
            </div>
        )
    },

    onClickFeed: function(feed_id, e) {
        e.preventDefault();
        this.props.onSelectFeed(feed_id);
    }
});

module.exports = FeedList;
