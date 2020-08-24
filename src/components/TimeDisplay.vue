<template>
<div :key="key">
  <div class="field">
    <label class="label">Date</label>
    <div class="control">
      <input class="input" v-model="currentDate" />
    </div>
    <div  class="field is-grouped">
      <div class="control is-expanded">
        <input class="slider is-fullwidth is-medium" step="1" min="-365" max="365" v-model="day" type="range">
      </div>
      <div class="control">
        <span
          class="icon is-small is-left tooltip"
          :class="toolTipClass"
          :data-tooltip="helpText.date"
          v-html="questionMark">
        </span>
      </div>
    </div>
  </div>
  <div class="field">
    <label class="label">Time</label>
    <div class="control">
      <input class="input" v-model="currentTime"/>
    </div>
    <div class="field is-grouped">
      <div class="control is-expanded">
        <input class="slider is-fullwidth is-medium" step="15" :min="-24*60" :max="24*60" v-model="minute" type="range">
      </div>
      <div class="control">
        <span
          class="icon is-small is-left tooltip"
          :class="toolTipClass"
          :data-tooltip="helpText.time"
          v-html="questionMark">
        </span>
      </div>
    </div>
  </div>
  <div class="field">
    <button class="button" @click="onClick">Now</button>
  </div>

  <!-- <div class="field is-horizontal">
    <div class="field-label is-normal">
      <label class="label">Date
      </label>
    </div>
    <div class="field field-body is-grouped">
      <div class="control is-expanded">
        <input class="input" v-model="currentDate"/>
      </div>
    </div>
  </div>
  <div class="field is-horizontal">
    <div class="field-label is-normal"></div>
    <div  class="field field-body is-grouped">
      <div class="control is-expanded">
        <input class="slider is-fullwidth" step="1" min="0" max="365" v-model="day" type="range">
      </div>
      <div class="control">
        <span
          class="icon is-small is-left tooltip"
          :class="toolTipClass"
          :data-tooltip="helpText.date"
          v-html="questionMark">
        </span>
      </div>
    </div>
  </div>
  <div class="field is-horizontal">
    <div class="field-label is-normal">
      <label class="label">Time</label>
    </div>
    <div class="field field-body is-grouped">
      <div class="control is-expanded">
        <input class="input" v-model="currentTime"/>
      </div>
    </div>
  </div>
  <div class="field is-horizontal">
    <div class="field-label is-normal">
    </div>
    <div class="field field-body is-grouped">
      <div class="control is-expanded">
        <input class="slider is-fullwidth" step="15" min="0" :max="24*60" v-model="minute" type="range">
      </div>
      <div class="control">
        <span
          class="icon is-small is-left tooltip"
          :class="toolTipClass"
          :data-tooltip="helpText.time"
          v-html="questionMark">
        </span>
      </div>
    </div>
  </div> -->
</div>
</template>

<script>

import moment from "moment"
import octicons from "octicons"


export default {
  props: {
    time: {type: Object, default: ()=>{return moment()}}
  },
  components: {
  },
  methods: {
    onResize(){
      this.detectMobile()
      if (this.key === 0){
        this.key = 1
      }else{
        this.key = 0
      }
    },
    reset(){
      this.day = 0
      this.minute = 0
      this.currentTime = this.time.format("HH:mm:ss")
      this.currentDate = this.time.format("YYYY/MM/DD")
    },
    onClick(){
      this.reset()
    },
    // onDatekeydown(evt){
    //   if (evt.key === 'Enter') {
    //
    //   }
    // },
    emitChangeEvent(){
      this.$emit("change", this.parseDateTime())
    },
    // onGetEphemeridesClick(){
    //   this.initialDateTime = this.parseDateTime()
    //   console.log(
    //     `GeoLocationTimeDisplay.onGetEphemeridesClick: lat: ${this.lat}, lon: ${this.lon}, elevation: ${this.elevation}`)
    //   console.log(
    //     `GeoLocationTimeDisplay.onGetEphemeridesClick: time: ${this.initialDateTime}`)
    //   this.$emit(
    //     "on-change",
    //     this.getGeoLocation(),
    //     this.initialDateTime
    //   )
    // },
    parseDateTime(){
      var dateTime = moment(
        `${this.currentDate} ${this.currentTime}`,
        "YYYY/MM/DD HH:mm:ss"
      )
      return dateTime
    },
    getGeoLocation(){
      return {
        lon: this.lon,
        lat: this.lat,
        elevation: this.elevation
      }
    },
    detectMobile(){
      if (window.innerWidth > 768){
        this.toolTipClass = {
          "is-tooltip-bottom": true
        }
      } else {
        this.toolTipClass = {
          "is-tooltip-left": true
        }
      }
    }
  },
  mounted(){
    // window.addEventListener('resize', this.onResize)
    this.$nextTick(this.onResize)
  },
  watch: {
    time(){
      this.currentTime = this.time.format("HH:mm:ss")
      this.currentDate = this.time.format("YYYY/MM/DD")
    },
    minute(){
      // minuteFlag = !minuteFlag
      var currentTimeObj = this.initialDateTime.clone()
      currentTimeObj.add(this.minute, "minutes")
      this.currentTime = currentTimeObj.format("HH:mm:ss")
      this.emitChangeEvent()
    },
    day(){
      // dayFlag = !dayFlag
      var currentTimeObj = this.initialDateTime.clone()
      currentTimeObj.add(this.day, "days")
      this.currentDate = currentTimeObj.format("YYYY/MM/DD")
      this.emitChangeEvent()
    },
    currentTime(){
      console.log(`TimeDisplay.watch.currentTime`)
      this.currentDateTime = this.parseDateTime()
      // let minutes = this.currentDateTime.diff(this.initialDateTime, "minutes")

    },
    currentDate(){
      console.log(`TimeDisplay.watch.currentDate`)
    },
    toolTipClass(){
      // console.log('toolTipClass: watch')
    }
  }
  data() {
    return {
      initialDateTime: this.time.clone(),
      currentDateTime: this.time.clone(),
      currentTime: this.time.format("HH:mm:ss"),
      currentDate: this.time.format("YYYY/MM/DD"),
      minuteFlag: false,
      minute: 0,
      dayFlag: false,
      day: 0,
      helpText: {
        date: "Move the slider to increment the date by 1 day",
        time: "Move the slider to increment the time by 15 minutes"
      },
      key: 0,
      toolTipClass: {
        'is-tooltip-bottom': true,
      },
      questionMark: octicons.question.toSVG()
    }
  }
}
</script>

<style scoped>
.control span {
  margin-top: 1.3rem;
}
</style>
