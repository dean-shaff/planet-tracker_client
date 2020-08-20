<template>
  <div>
    <table class="table is-hoverable" :key="key" >
      <thead>
        <td>Name</td>
        <td>Azimuth/ Elevation</td>
        <!-- <td><a href="https://en.wikipedia.org/wiki/Apparent_magnitude" target="_blank"><em>m</em></a></td> -->
        <!-- <td v-show="detectMobile()">Setting Time</td>
        <td v-show="detectMobile()">Rising Time</td> -->
        <td>Setting Time</td>
        <td>Rising Time</td>

      </thead>
      <tbody class="tbody" @mouseout="onMouseout">
        <tr v-for="(name, index) in Object.keys(astronDisplayData)" @mouseover="onMouseover(name, index)">
          <td>{{name}}</td>
          <td>{{astronDisplayData[name].az}}&deg;/{{astronDisplayData[name].el}}&deg;</td>
          <!-- <td>{{astronDisplayData[name].magnitude}}</td> -->
          <!-- <td v-show="detectMobile()">{{astronDisplayData[name].setting_time}}</td>
          <td v-show="detectMobile()">{{astronDisplayData[name].rising_time}}</td> -->
          <td>{{astronDisplayData[name].setting_time}}</td>
          <td>{{astronDisplayData[name].rising_time}}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>

import moment from "moment"

import util from "./../util.js"

export default {
  props: {
    astronObjects: {type: Object, default: ()=>{return {}}}
  },
  methods: {
    // onClick(name){
    //   console.log(JSON.stringify(this.astronObjects[name]))
    // },
    onResize(){
      if (this.key == 0) {
        this.key = 1
      } else {
        this.key = 0
      }
    },
    updateDisplayData(){
      var displayData = {}
      const pyEphemStrFormat = "YYYY/M/D HH:mm:ss"
      var decimalPlaces = 0
      if (this.detectMobile()) {
        decimalPlaces = 2
      }
      Object.keys(this.astronObjects).forEach((name)=>{
        var displayObject = Object.assign({}, this.astronObjects[name])

        let settingTime = displayObject.setting_time
        let settingTimeMoment = moment.utc(settingTime, pyEphemStrFormat)

        let risingTime = displayObject.rising_time
        let risingTimeMoment = moment.utc(risingTime, pyEphemStrFormat)

        if (displayObject.el < 0) {
          displayObject.setting_time = "-"
          displayObject.rising_time = risingTimeMoment.local().format("HH:mm")
        } else {
          displayObject.setting_time = settingTimeMoment.local().format("HH:mm")
          displayObject.rising_time = "-"
        }

        this.formattableFields.forEach((field)=>{
          var fieldVal = displayObject[field]
          if (typeof fieldVal === "number"){
            displayObject[field] = util.radToDegree(fieldVal).toFixed(decimalPlaces)
          }
        })


        displayData[name] = displayObject
      })
      return displayData
    },
    detectMobile(){
      return window.innerWidth > 768
    },
    onMouseover(name, index){
      this.$emit("on-hover", index, name)
    },
    onMouseout(){
      this.$emit("on-mouseout")
    }
  },
  computed: {
    astronDisplayData: function(){
      return this.updateDisplayData()
    }
  },
  mounted(){
    window.addEventListener('resize', this.onResize)
  },
  data(){
    return {
      formattableFields: ["az", "el", "ra", "dec"],
      key: 0
    }
  }
}
</script>

<style scoped>
</style>
