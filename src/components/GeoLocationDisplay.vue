<template>
<div
  :style="{'width': width + 'px', 'height': height + 'px'}"
  ref="geo-location-display">
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
      map.on("contextmenu", this.onContextmenu())

      let locationControl = L.control.locate({
        keepCurrentZoomLevel: true
      }).addTo(map)
      locationControl.start()
      // locationControl.stopFollowing()

      this.locationControl = locationControl

      this.map = map
      this.marker = null

      let hereSelector = this.$refs['geo-location-display'].querySelector("a.leaflet-bar-part")
      console.log(hereSelector, hereSelector.constructor)
      hereSelector.onclick = this.onHereClick(false)

      // /html/body/div[1]/section/div/div[3]/div[1]/div[2]/div[1]/div[2]/div[1]/div[2]/a
      // let marker = L.marker([lat, lon]).addTo(map);
      // this.marker = marker
    },
    updateGeoLocationFromLatLng(latlng) {
      let geoLocation = {
        "lat": latlng.lat,
        "lon": latlng.lng,
        "elevation": 0.0
      }
      this.geoLocation = Object.assign({}, geoLocation)
    },
    onHereClick(toggle){
      return () => {
        // console.log(`GeoLocationDisplay.onHereClick: ${toggle}`)
        // console.log(this.map.getCenter())
        if (toggle) {
          this.geoLocation = Object.assign({}, this.detectedGeoLocation)
          this.updateOnLocationfound = true
          if (this.marker != null) {
            this.map.removeLayer(this.marker)
            this.marker = null
          }
        }
        toggle = !toggle
      }
    },
    onLocationfound(){
      return (evt) => {
        console.log(`GeoLocationDisplay.onLocationfound`)
        if (this.updateOnLocationfound) {
          let latlng = evt.latlng
          this.updateGeoLocationFromLatLng(latlng)
          this.detectedGeoLocation = Object.assign({}, this.geoLocation)
          this.map.setView([latlng.lat, latlng.lng])
          this.updateOnLocationfound = false
        }
      }
    },
    onContextmenu(){
      return (evt) => {
        console.log(`GeoLocationDisplay.onContextmenu`)
        let latlng = evt.latlng
        this.updateGeoLocationFromLatLng(latlng)
        if (this.marker == null) {
          this.marker = L.marker([latlng.lat, latlng.lng]).addTo(this.map)
        } else {
          this.marker.setLatLng(latlng)
        }
      }
    }
  },
  watch: {
    geoLocation(newGeoLocation, oldGeoLocation) {
      let same = true
      Object.keys(oldGeoLocation).forEach((key) => {
        if (oldGeoLocation[key] !== newGeoLocation[key]) {
          same = false
        }
      })
      if (! same) {
        this.$emit("change", newGeoLocation)
      }
    }
  },
  data: function () {
    return {
      "geoLocation": {lat: 0.0, lon: 0.0, elevation: 0.0},
      "detectedGeoLocation": {lat: 0.0, lon: 0.0, elevation: 0.0},
      "map": null,
      "marker": null,
      "updateOnLocationfound": true
    }
  }
}

</script>
