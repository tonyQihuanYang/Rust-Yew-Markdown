### RUN

```
trunk serve
```

###### Run in Pm2

```
 pm2 start --name "Yew Markdown" ./start.sh
```

### Libraries

- Markdown parser [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark/blob/master/examples/string-to-string.rs)
- Rustwasm warpper for the web-sys, feature timer [gloo-timers](https://github.com/rustwasm/gloo)
- used for wasm-bindgen-futures, spawn_local[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/examples/fetch.html)
- GraphQL [graphQl-client](https://docs./graphql_client/latest/graphql_client/)

### Styles

- [Bootstrap 4](https://hackerthemes.com/bootstrap-cheatsheet/#navbar-light)
- [Style Reference](https://github.com/yewstack/yew/issues/533)
- [Drawer](https://codepen.io/rocknpx/pen/oEEEZX)

### DOCKER

```
docker build -t yew-markdown .

```

### GraphQL

```
query {
	markdowns {
    title,
    context
  }
}


mutation {
  createMarkdown(
    newMarkdown: { title: "Fresh Kid Ice", context: "## what" }
  ) {
    title
    context
  }

}

```
