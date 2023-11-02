# UrlTown

https://urltown.apps.loskutoff.com/

- On-page wasm continuous async simulation 
- Done in Bevy game engine
- Integrated with the page itself in the form of interactive navigation menu
- Rust and Typescript code is integrated inside a monorepo
- An external editor used as a "CMS" (LDTK)
- Tilemap based animation
- Collisions
- Pathfinding

## Assets

tiny town tileset: https://kenney.nl/assets/tiny-town Creative Commons CC0
sprout lands animation: https://cupnooble.itch.io/sprout-lands-asset-pack (This asset pack can be used in any non-commercial project, https://cupnooble.itch.io/)

## Run locally for development


handle assets: 

`ln -s ~/work/blog/nxblog/packages/town/assets packages/astroblog/public/assets`

`npx nx serve astroblog`

## Build and run

build and run web server from dist/packages/astroblog i.e. 

`npx nx build town`

`npx nx build astroblog`

`cd dist/packages/astroblog && python3 -m http.server`

## Deploy

`git push dokku master`

## TODOS

TODO doesn't work well with the "back" button of iOS safari - probably don't get the "unfocus" event

TODO build: no auto build; do manually the town (nx build town) , then site (nx build astroblog), then copy assets dir right into astroblog build root `cp -R packages/town/assets dist/packages/astroblog`; it's baked in

TODO until this https://github.com/Cammisuli/monodon/pull/22 is resolved, I check out my fork and use it as dependency or as an alternative symlink it into packages: `ln -s ~/work/blog/monodon/packages/rust`

TODO topLevelAwait() plugin just crashes build without no useful error indication

TODO https://github.com/bevyengine/bevy/issues/3800 - size optimisations
