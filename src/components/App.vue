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
                <div class="level-right">
                    <div class="level-item">
                        <a href="https://github.com/dean-shaff/planet-tracker" target="_blank">
                            <h1 class="title is-1">Source</h1>
                        </a>
                    </div>
                </div>
            </div>
            <div class="is-divider"></div>
            <div class="columns">
                <div class="column is-one-third">
                    <div class="level">
                        <div class="level-item">
                            <h3 class="subtitle is-3">Geo Location and Time</h3>
                        </div>
                    </div>
                    <geo-location-time-display
                        :time="currentTime"
                        :geoLocation="geoLocation"
                        @on-change="onChange"
                        @on-here="onHere">
                    </geo-location-time-display>
                    <div class="is-divider"></div>
                    <div class="level">
                        <div class="level-item">
                            <h3 class="subtitle is-3">Planet Ephemerides</h3>
                        </div>
                    </div>
                    <astron-text-display
                        :astronObjects="astronObjects">
                    </astron-text-display>
                </div>
                <div class="column" ref="polar-plot-container">
                    <d3-polar-plot
                        :circles="astronPlotData"
                        :circleOptions="astronPlotOptions"
                        :width="polarPlotWidth"
                        :height="polarPlotHeight"
                        :key="polarPlotKey">
                    </d3-polar-plot>
                </div>
            </div>
            <!-- <div class="is-size-5" v-html="status"></div> -->
            <div class="is-size-6">Version {{version}}</div>
        </div>
    </section>
</div>
</template>

<script>

import io from "socket.io-client"
import Vue from "vue"
import moment from "moment"

// import TimeDisplay from "./TimeDisplay.vue"
// import GeoLocationDisplay from "./GeoLocationDisplay.vue"
import GeoLocationTimeDisplay from "./GeoLocationTimeDisplay.vue"
import AstronTextDisplay from "./AstronTextDisplay.vue"
import D3PolarPlot from "./D3PolarPlot.vue"

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
        // "time-display": TimeDisplay,
        // "geo-location-display": GeoLocationDisplay
        "geo-location-time-display": GeoLocationTimeDisplay
    },
    methods:{
        init(){
            console.log("App.init")
            this.requestGeoLocation(
            ).then(this.setGeoLocation
            ).then((geoLocation)=>{
                this.currentTime = moment.utc()
                return this.requestAstronCoordinates(geoLocation)
            }).catch(this.geoLocationError)
        },
        requestAstronCoordinates(geoLocation){
            Object.keys(this.astronObjects).forEach((name)=>{
                this.socket.emit("get_astron_object_data", {
                    name: name,
                    when: this.currentTime.format(),
                    cb_name: "get_astron_object_data_handler",
                    geo_location: Object.assign({}, geoLocation)
                })
            })
        },
        setGeoLocation(position){
            if ("coords" in position){
                this.status = "This browser supports geolocation"
                this.geoLocation = {
                    lat: position.coords.latitude,
                    lon: position.coords.longitude,
                    elevation: position.coords.altitude
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
            console.log("App.getAstronObjectData")
            var name = data.name
            var astronObjectsCopy = Object.assign({}, this.astronObjects)
            astronObjectsCopy[name] = data
            this.astronObjects = Object.assign({}, astronObjectsCopy)
            // Vue.set(this.astronObjects, name, data)
            // console.log("getAstronObjectData")
            // console.log(this.astronObjects)
            // this.astronObjects = Object.assign(this.astronObjects, this.astronObjects[name], data)
            // console.log(this.astronObjects)
        },
        registerSocketHandlers(socket){
            socket.on("connect", this.init)
            socket.on("get_astron_object_data_handler", this.getAstronObjectData)
        },
        onChange(newGeoLocation, newTime){
            console.log(`App.onChange`)
            this.currentTime = newTime
            this.geoLocation = Object.assign(this.geoLocation, newGeoLocation)
            this.requestAstronCoordinates(this.geoLocation)
        },
        onHere(){
            console.log(`App.onHere`)
            this.requestGeoLocation(
            ).then(this.setGeoLocation
            ).then((geoLocation)=>{
                return this.requestAstronCoordinates(geoLocation)
            }).catch(this.geoLocationError)
        },

        reRenderPolarPlot(){
            var width = this.$refs["polar-plot-container"].offsetWidth
            this.polarPlotWidth = width
            this.polarPlotHeight = width
            if (this.polarPlotKey === 0){
                this.polarPlotKey = 1
            }else{
                this.polarPlotKey = 0
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
        // console.log(this.host, this.port)
        // this.socket = io(`https://${this.host}:${this.port}`)
        this.socket = io()
        this.registerSocketHandlers(this.socket)
        this.reRenderPolarPlot()
        window.addEventListener('resize', this.reRenderPolarPlot)
    },
    destroyed(){
        this.socket.disconnect()
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
            currentTime: moment.utc(),
            geoLocation: {lat: 0.0, lon: 0.0, elevation: 0.0},
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
                "Pluto": {},
            },
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
                "Pluto": {r: 10, class: "scatter", stroke: 1.0, fill: planetFill["Pluto"], opacity: visibleOpacity}
            },
            underHorizonFill: "rgba(180, 180, 180)",
            "visibleOpacity": visibleOpacity,
            "underHorizonOpacity": underHorizonOpacity,
            horizon: 0.0,
            astronPlotData: [],
            polarPlotWidth: 100,
            polarPlotHeight: 100,
            polarPlotKey: 0
        }
    }
}
</script>


<style>
.field-label{
    flex-grow: 2;
}
</style>
