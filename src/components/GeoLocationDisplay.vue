<template>
<div
  :style="{'width': width + 'px', 'height': height + 'px'}">
</div>
</template>


<script>
import L from 'leaflet';
import 'leaflet.locatecontrol'

import 'leaflet/dist/leaflet.css'
import 'leaflet.locatecontrol/dist/L.Control.Locate.min.css'

import '@fortawesome/fontawesome-free/css/all.css'

/* This code is needed to properly load the images in the Leaflet CSS */
delete L.Icon.Default.prototype._getIconUrl;
L.Icon.Default.mergeOptions({
  iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
  iconUrl: require('leaflet/dist/images/marker-icon.png'),
  shadowUrl: require('leaflet/dist/images/marker-shadow.png'),
});


export default {
  props: {
    height: {type: Number, default:300},
    width: {type: Number, default:300},
    zoom: {type: Number, default: 13},
    mode: {type: String, default: "mapbox"}
  },
  mounted: function () {
    this.createMap(this.$el)
    // console.log(`TileMap.mounted: width=${this.width}, height=${this.height}`)
  },
  methods: {
    createMap(elem) {
      const [lat, lon] = [this.geoLocation.lat, this.geoLocation.lon]
      const token = "pk.eyJ1IjoidW5qdXNkb3JhbmdlIiwiYSI6ImNrZTh1bng2ajIwN20yc213MjAwcXVjejgifQ.6PxebkbtzOBGYNMNl9rY3Q"
      console.log(`TileMap.createMap: lat=${lat}, lon=${lon}`)
      let map = L.map(elem).setView([lat, lon], this.zoom);

      if (this.mode == "osm") {
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
          maxZoom: 19,
          attribution: '&copy; <a href="https://openstreetmap.org/copyright">OpenStreetMap contributors</a>'
        }).addTo(map);
      } else if (this.mode == "mapbox") {
        L.tileLayer(`https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token=${token}`, {
          attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
          maxZoom: 18,
          id: 'mapbox/streets-v10',
          tileSize: 512,
          zoomOffset: -1,
          accessToken: token
        }).addTo(map);
      }


      map.on("locationfound", this.onLocationfound())
      // map.on("click", this.onClick())
      map.on("contextmenu", this.onContextmenu())

      let locationControl = L.control.locate({
        keepCurrentZoomLevel: true
      }).addTo(map)
      locationControl.start()

      this.map = map
      // let marker = L.marker([lat, lon]).addTo(map);
      // this.marker = marker
    },
    updateGeoLocationFromLatLng(latlng) {
      let geoLocation = {
        "lat": latlng.lat,
        "lon": latlng.lng,
        "elevation": 0.0
      }
      let same = true
      Object.keys(geoLocation).forEach((key) => {
        if (geoLocation[key] !== this.geoLocation[key]) {
          same = false
        }
      })
      if (! same) {
        this.$emit("change", geoLocation)
        this.geoLocation = geoLocation
      }
    },
    onClick(){
      return () => {
        console.log(`GeoLocationDisplay.onClick`)
      }
    },
    onLocationfound(){
      return (evt) => {
        let latlng = evt.latlng
        this.updateGeoLocationFromLatLng(latlng)
      }
    },
    onContextmenu(){
      return (evt) => {
        console.log(`GeoLocationDisplay.onContextmenu`)
        let latlng = evt.latlng
        this.map.setView([latlng.lat, latlng.lng]) //, this.zoom)
        this.updateGeoLocationFromLatLng(evt.latlng)
      }
    }
  },
  data: function () {
    return {
      "geoLocation": {lat: 0.0, lon: 0.0, elevation: 0.0},
      "map": null,
      "marker": null
    }
  }
}

</script>
