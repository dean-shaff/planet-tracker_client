import Vue from "vue"

import App from "./components/App.vue"

const version = "3.1.1"

var init = ()=>{
  var app = new Vue({
    components:{
      "app":App
    },
    data(){
      return {
        port: "5000",
        host: document.domain,
        "version": version
      }
    },
    el: "#app",
    template:`<app ref="app" :port="port" :host="host" :version="version"></app>`
  })
  return app
}

var app = init()
window.app = app
