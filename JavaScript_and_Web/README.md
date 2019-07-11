# Webassembly with wasm-bidgen

First, follow [the examples](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html).

You will find that what you need are just webassembly and js file. Then, refer the code below wherever you want to integrate it.

```js
const web = import("./javascript_and_web");

web.then(fn => {
  fn.create_stuff();
});
```

or entire example with React

```js
import React, { Component } from "react";
import PropTypes from "prop-types";

export default class YouTubeButton extends Component {
  constructor(props) {
    super(props);
    this.youtubeShareNode = React.createRef();
  }

  state = {
    initialized: false,
  }

  static propTypes = {
    // channelName: PropTypes.string,
    channelid: PropTypes.string.isRequired,
    channelTitle: PropTypes.string,
    layout: PropTypes.string,
    theme: PropTypes.string,
    count: PropTypes.string,
  };

  static defaultProps = {
    // channelName: "", YouTube API don't care about channelName
    channelid: "UCt_jsJOe91EVjd58kHpgTfw",
    channelTitle: "Steadylearner",
    layout: "full",
    theme: "default",
    count: "default",
  };

  initialized() {
    this.setState({
      initialized: true,
    });
  }

  componentDidMount() {
    if (this.state.initialized) {
      return;
    }

    const rust = import("./web/javascript_and_web");

    rust.then(({ create_stuff }) => {
      create_stuff();
    }).then(() => {
      const youtubescript = document.getElementById("youtube-subscribe");
      this.youtubeShareNode.current.parentNode.appendChild(youtubescript);
      this.initialized();
    }).catch((e) => {
      console.error("failed to read wasm file and use javascript instead", e);
      const youtubescript = document.createElement("script");
      youtubescript.src = "https://apis.google.com/js/platform.js";
      this.youtubeShareNode.current.parentNode.appendChild(youtubescript);
      this.initialized();
    });
  }

  shouldComponentUpdate(nextProps, nextState) {

    if (this.props.channelid === nextProps.channelid) {
      return false;
    }
    return true;
  }

  render() {
    const {
      channelid,
      channelTitle = "Stadylearner",
      layout,
      theme,
      count,
    } = this.props;
    return (
      <>
        <div
          className="g-ytsubscribe left-auto"
          ref={this.youtubeShareNode}
          data-theme={theme}
          data-layout={layout}
          data-count={count}
          data-channelid={channelid}
          // data-channel={channelName}
        />
      </>
    );
  }
}
```

You don't need node_modules and javascript files except that you need to test them work or not at the browser.

For optimization, you may refer to [this repository](https://github.com/WebAssembly/binaryen).

You may refer to [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html)
and [wasm-pack](https://github.com/rustwasm/wasm-pack).