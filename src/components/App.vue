<template>
<div>
  <section class="section">
    <div class="container">
      <div class="level">
        <div class="level-left">
          <div class="level-item">
            <h1 class="title is-1">Planet Tracker</h1>
          </div>
        </div>
      </div>
      <div class="is-divider"></div>
      <div class="columns">
        <div class="column is-one-third">
          <div class="level">
            <div class="level-left">
              <div class="level-item">
                <!-- <h3 class="subtitle is-5">Geo Location</h3> -->
                <label class="label">Geo Location</label>
              </div>
            </div>
            <div class="level-right">
              <div class="level-item">
                <span
                  class="icon is-small is-left tooltip"
                  :data-tooltip="helpText.geoLocation"
                  v-html="questionMark">
                </span>
              </div>
            </div>
          </div>
          <div ref="geo-location-time-display-container">
            <geo-location-display
              :width="geoLocationDisplayWidth"
              :height="geoLocationDisplayHeight"
              :key="geoLocationDisplayKey"
              mode="mapbox"
              :zoom="5"
              @change="onGeoLocationDisplayChange">
            </geo-location-display>
            <div class="is-divider"></div>
            <time-display
              :time="currentTime"
              @change="onTimeDisplayChange">
            </time-display>
          </div>

          <!-- <geo-location-time-display
            :time="currentTime"
            :geoLocation="geoLocation"
            :geoLocationDisplayWidth="geoLocationDisplayWidth"
            :geoLocationDisplayHeight="geoLocationDisplayHeight"
            :geoLocationDisplayKey="geoLocationDisplayKey"
            @on-change="onChange"
            @on-here="onHere"
            @on-geolocation="onGeolocation">
          </geo-location-time-display> -->

          <div class="is-divider"></div>
          <div class="level">
            <div class="level-item">
              <h3 class="subtitle is-3">Planet Ephemerides</h3>
            </div>
          </div>
          <astron-text-display
            :astronObjects="astronObjects"
            @on-hover="onAstronTextDisplayHover"
            @on-mouseout="onAstronTextDisplayMouseout">
          </astron-text-display>
        </div>
        <div class="column" ref="polar-plot-container">
          <d3-polar-plot
            :circles="astronPlotData"
            :circleOptions="astronPlotOptions"
            :width="polarPlotWidth"
            :height="polarPlotHeight"
            :key="polarPlotKey"
            :tooltipTarget="astronPlotTooltipTarget">
          </d3-polar-plot>
        </div>
      </div>
      <div class="is-divider"></div>
      <!-- <div class="is-size-5" v-html="status"></div> -->
      <!-- <div class="level">
        <div class="is-size-6 level-item has-text-centered">Version {{version}}</div>
      </div>
      <div class="level">
        <a href="https://github.com/dean-shaff/planet-tracker" target="_blank" class="level-item has-text-centered">
        <div class="is-size-6 red-hover">
          Source
        </div>
        </a>
      </div> -->
      <div class="level">
        <div class="is-size-6">Version {{version}}</div>
        <a href="https://github.com/dean-shaff/planet-tracker" target="_blank">
          <div class="is-size-6 red-hover">Source</div>
        </a>
      </div>

    </div>
  </section>
</div>
</template>

<script>
import Vue from "vue"
import moment from "moment"
import octicons from "octicons"

import AstronTextDisplay from "./AstronTextDisplay.vue"
import D3PolarPlot from "./D3PolarPlot.vue"
import GeoLocationDisplay from "./GeoLocationDisplay.vue"
import TimeDisplay from "./TimeDisplay.vue"



import util from "./../util.js"

export default {
  props: {
    host: {type: String, default: "localhost"},
    port: {type: String, default: "5000"},
    version: {type: String, default: ""}
  },
  components:{
    "astron-text-display": AstronTextDisplay,
    "d3-polar-plot": D3PolarPlot,
    "geo-location-display": GeoLocationDisplay,
    "time-display": TimeDisplay
  },
  methods:{
    init(){
      console.log("App.init")
      // return this.requestGeoLocation(
      // ).then(this.setGeoLocation
      // ).then((geoLocation)=>{
      //   this.currentTime = moment()
      //   return this.requestAstronCoordinates(geoLocation, this.currentTime)
      // })
    },
    get(url, data){
      var request = new XMLHttpRequest()
      if (data !== undefined){
        url += `?${data}`
      }
      // console.log(`get: ${url}`)
      request.open("GET", url, true)
      return new Promise((resolve, reject)=>{
        request.send()
        request.onreadystatechange = function(){
          if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
            resolve(this)
          }
        }
        request.onerror = reject
      })
    },
    requestAstronCoordinates(geoLocation, currentTime) {
      currentTime = moment.utc(currentTime)
      console.log(`App.requestAstronCoordinates: ${currentTime.format()}`)
      var promises = Object.keys(this.astronObjects).map((name)=>{
        var reqData = {
          name: name,
          when: currentTime.format(),
          lon: geoLocation.lon,
          lat: geoLocation.lat,
          elevation: geoLocation.elevation
        }
        var reqDataUrlArr = Object.keys(reqData).map((c) => {
          return `${c}=${reqData[c]}`
        })
        // console.log(reqDataUrlArr)
        this.get(
          "./get_astron_object_data",
          reqDataUrlArr.join("&")
        ).then((req) => {
          var data = JSON.parse(req.response)
          this.getAstronObjectData(data)
        })
      })
      return Promise.all(promises)
    },
    setGeoLocation(position){
      if ("coords" in position){
        this.status = "This browser supports geolocation"
        var altitude = position.coords.altitude
        this.geoLocation = {
          lat: position.coords.latitude,
          lon: position.coords.longitude,
          elevation: altitude === null ? 0.0: altitude
        }
      }else{
        this.status = "Browser doesn't support geolocation"
      }
      return this.geoLocation
    },
    requestGeoLocation(){
      if (navigator.geolocation){
        return new Promise(
          (resolve, reject)=>{navigator.geolocation.getCurrentPosition(resolve, reject)}
        )
      }else{
        return new Promise(
          (resolve)=>{resolve({})}
        )
      }
    },
    geoLocationError(err){
      console.error(err)
      this.status = err.message
    },
    getAstronObjectData(data){
      var name = data.name
      // console.log(`App.getAstronObjectData: ${name}`)
      var astronObjectsCopy = Object.assign({}, this.astronObjects)
      astronObjectsCopy[name] = data
      this.astronObjects = Object.assign({}, astronObjectsCopy)
      // Vue.set(this.astronObjects, name, data)
      // console.log("getAstronObjectData")
      // console.log(this.astronObjects)
      // this.astronObjects = Object.assign(this.astronObjects, this.astronObjects[name], data)
      // console.log(this.astronObjects)
    },
    onAstronTextDisplayHover(index, name) {
      console.log(`App.onAstronTextDisplayHover: index=${index}, name=${name}`)
      this.astronPlotTooltipTarget = [index, name]
    },
    onAstronTextDisplayMouseout() {
      console.log(`App.onAstronTextDisplayMouseout`)
      this.astronPlotTooltipTarget = null
    },
    // onChange(newGeoLocation, newTime){
    //   console.log(`App.onChange`)
    //   this.currentTime = newTime
    //   this.geoLocation = Object.assign(this.geoLocation, newGeoLocation)
    //   this.requestAstronCoordinates(this.geoLocation, this.currentTime)
    // },
    // onHere(){
    //   console.log(`App.onHere`)
    //   this.requestGeoLocation(
    //   ).then(this.setGeoLocation
    //   ).then((geoLocation)=>{
    //     return this.requestAstronCoordinates(geoLocation, this.currentTime)
    //   }).catch(this.geoLocationError)
    // },
    // onGeolocation(newGeoLocation, newTime){
    //   console.log(`App.onGeolocation`)
    //   this.currentTime = newTime
    //   this.geoLocation = Object.assign(this.geoLocation, newGeoLocation)
    //   this.requestAstronCoordinates(this.geoLocation, this.currentTime)
    // },
    onTimeDisplayChange(newTime){
      console.log(`App.onTimeDisplayChange`)
      this.currentTime = newTime
      this.requestAstronCoordinates(this.geoLocation, this.currentTime)
    },
    onGeoLocationDisplayChange(newGeoLocation){
      console.log(`App.onGeoLocationDisplayChange`)
      this.geoLocation = Object.assign(this.geoLocation, newGeoLocation)
      this.requestAstronCoordinates(this.geoLocation, this.currentTime)
    },
    reRenderPolarPlot(){
      let width = this.$refs["polar-plot-container"].offsetWidth
      console.log(`App.reRenderPolarPlot: width=${width}`)
      this.polarPlotWidth = width
      this.polarPlotHeight = width
      if (this.polarPlotKey === 0){
        this.polarPlotKey = 1
      }else{
        this.polarPlotKey = 0
      }
    },
    reRenderGeoLocationTimeDisplay() {
      let width = this.$refs["geo-location-time-display-container"].offsetWidth
      this.geoLocationDisplayWidth = width
      this.geoLocationDisplayHeight = width
      if (this.geoLocationDisplayKey === 0){
        this.geoLocationDisplayKey = 1
      }else{
        this.geoLocationDisplayKey = 0
      }
    },
    calculateSizeFromMagnitude(magnitude){
      magnitude *= -1
      return magnitude + 17
    }
  },
  watch:{
    astronObjects:{
      handler: function(){
        var astronPlotData = []
        this.astronPlotData = Object.keys(this.astronObjects).forEach((name)=>{
          var obj = Object.assign({}, this.astronObjects[name])
          if ("magnitude" in obj){
            this.astronPlotOptions[name].r = this.calculateSizeFromMagnitude(obj.magnitude)
          }
          if ("az" in obj){
            obj.az = util.radToDegree(obj.az)
            obj.el = util.radToDegree(obj.el)
            if (obj.el < this.horizon){
              obj.el *= -1
              this.astronPlotOptions[name].opacity = this.underHorizonOpacity
              this.astronPlotOptions[name].fill = this.underHorizonFill
            }else{
              this.astronPlotOptions[name].opacity = this.visibleOpacity
              this.astronPlotOptions[name].fill = this.planetFill[name]
            }
            astronPlotData.push(obj)
          }
        })
        this.astronPlotData = astronPlotData
      },
      deep: true
    }
  },
  mounted(){
    // this.reRenderPolarPlot()
    // window.addEventListener('resize', this.reRenderPolarPlot)
    this.reRenderPolarPlot()
    this.reRenderGeoLocationTimeDisplay()
    window.addEventListener('resize', () => {this.reRenderPolarPlot(); this.reRenderGeoLocationTimeDisplay()})
    // this.init().catch((err) => {
    //   console.error(`Error: ${err}`)
    // })
  },
  destroyed(){
    // this.socket.close()
  },
  data(){
    var planetFill = {
      "Sun": "rgb(255,204,0)",
      "Mercury": "rgb(215,179,119)",
      "Venus": "rgb(171,99,19)",
      "Mars": "rgb(114,47,18)",
      "Moon": "rgba(128,128,128)",
      "Jupiter": "rgb(150,81,46)",
      "Saturn": "rgb(215,179,119)",
      "Uranus": "rgb(195,233,236)",
      "Neptune": "rgb(71,114,255)",
      "Pluto": "rgba(128,128,128)"
    }
    var visibleOpacity = 0.8
    var underHorizonOpacity = 0.4
    return {
      currentTime: moment(),
      geoLocation: {lat: 0.0, lon: 0.0, elevation: 0.0},
      geoLocationDisplayWidth: 100,
      geoLocationDisplayHeight: 100,
      geoLocationDisplayKey: 0,
      status: "",
      socket: null,
      astronObjects: {
        "Sun": {},
        "Mercury": {},
        "Venus": {},
        "Mars": {},
        "Moon": {},
        "Jupiter": {},
        "Saturn": {},
        "Uranus": {},
        "Neptune": {},
        // "Pluto": {},
      },
      astronPlotTooltipTarget: null,
      "planetFill": planetFill,
      astronPlotOptions: {
        "Sun": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Sun"], opacity: visibleOpacity},
        "Mercury": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Mercury"], opacity: visibleOpacity},
        "Venus": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Venus"], opacity: visibleOpacity},
        "Mars": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Mars"], opacity: visibleOpacity},
        "Moon": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Moon"], opacity: visibleOpacity},
        "Jupiter": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Jupiter"], opacity: visibleOpacity},
        "Saturn": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Saturn"], opacity: visibleOpacity},
        "Uranus": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Uranus"], opacity: visibleOpacity},
        "Neptune": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Neptune"], opacity: visibleOpacity},
        // "Pluto": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Pluto"], opacity: visibleOpacity}
      },
      underHorizonFill: "rgba(180, 180, 180)",
      "visibleOpacity": visibleOpacity,
      "underHorizonOpacity": underHorizonOpacity,
      horizon: 0.0,
      astronPlotData: [],
      polarPlotWidth: 100,
      polarPlotHeight: 100,
      polarPlotKey: 0,
      toolTipClass: {
        'is-tooltip-bottom': true,
      },
      questionMark: octicons.question.toSVG(),
      helpText: {
        geoLocation: "Right click on map to calculate new ephemerides"
      }
    }
  }
}
</script>


<style>
.field-label{
  flex-grow: 2;
}

/* .level-item span {
  margin-top: -0.4rem;
} */
</style>
