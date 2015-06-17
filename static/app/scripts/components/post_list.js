var React = require("react"),
    Fluxxor = require("fluxxor"),
    FluxMixin = Fluxxor.FluxMixin(React),
    StoreWatchMixin = Fluxxor.StoreWatchMixin;

var PostList = React.createClass({
    mixins: [FluxMixin],

    render: function() {
        var posts = this.props.posts.map(function(post) {
            return (
                <div>
                    <h1>{post.title}</h1>
                    <div dangerouslySetInnerHTML={{__html: post.description}} />
                </div>
            );
        }.bind(this));
        return (
            <div className="posts">
                <h5>Posts</h5>
                {posts}
            </div>
        )
    },
});

module.exports = PostList;
