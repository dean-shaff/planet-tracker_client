<template>
<div :key="key">
  <div class="field">
    <label class="label">Date</label>
    <div class="control">
      <input class="input" @change="onDateChange" :value="currentDate"/>
    </div>
    <div  class="field is-grouped">
      <div class="control is-expanded">
        <input
          ref="day-slider"
          class="slider is-fullwidth is-medium"
          step="1"
          min="-365"
          max="365"
          @change="onDayChange"
          :value="days"
          type="range">
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
      <input class="input" :value="currentTime" @change="onTimeChange"/>
    </div>
    <div class="field is-grouped">
      <div class="control is-expanded">
        <input
          ref="minute-slider"
          class="slider is-fullwidth is-medium"
          step="15"
          :min="-24*60"
          :max="24*60"
          @change="onMinuteChange"
          :value="minutes"
          type="range">
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
      this.currentDateTime = this.initialDateTime.clone()
      // this.day = 0
      // this.minute = 0
      // this.currentTime = this.time.format("HH:mm:ss")
      // this.currentDate = this.time.format("YYYY/MM/DD")
    },
    onClick(){
      this.reset()
    },
    onDayChange(evt) {
      console.log(`TimeDisplay.onDayChange`)
      let days = evt.target.value
      let currentDateTime = this.initialDateTime.clone()
      this.currentDateTime = currentDateTime.add(days, "days").add(this.minutes, "minutes")
    },
    onDateChange(evt) {
      let dateComponent = evt.target.value
      let timeComponent = this.currentTime
      console.log(`TimeDisplay.onDateChange: ${dateComponent} ${timeComponent}`)
      this.currentDateTime = this.parseDateTime(dateComponent, timeComponent)
    },
    onMinuteChange(evt) {
      console.log(`TimeDisplay.onMinuteChange`)
      let minutes = evt.target.value
      let currentDateTime = this.initialDateTime.clone()
      this.currentDateTime = currentDateTime.add(minutes, "minutes").add(this.days, "days")
    },
    onTimeChange(evt) {
      console.log(`TimeDisplay.onTimeChange`)
      let timeComponent = evt.target.value
      let dateComponent = this.currentDate
      console.log(`TimeDisplay.onDateChange: ${dateComponent} ${timeComponent}`)
      this.currentDateTime = this.parseDateTime(dateComponent, timeComponent)
    },
    emitChangeEvent(currentDateTime){
      this.$emit("change", currentDateTime)
    },
    parseDateTime(date, time){
      let dateTime = moment(
        `${date} ${time}`,
        "YYYY/MM/DD HH:mm:ss"
      )
      return dateTime
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
  computed:{
    currentTime(){
      console.log("TimeDisplay.computed.currentTime")
      return this.currentDateTime.format("HH:mm:ss")
    },
    currentDate(){
      console.log("TimeDisplay.computed.currentDate")
      let val = this.currentDateTime.format("YYYY/MM/DD")
      return val
    },
    minutes(){
      let minutes = this.currentDateTime.diff(this.initialDateTime.add(this.days, "days"), "minutes")
      console.log(`TimeDisplay.computed.minutes: ${minutes}`)
      return minutes
    },
    days(){
      let hours = this.currentDateTime.diff(this.initialDateTime, "hours")
      let days = (hours + 1)/ 24
      console.log(`TimeDisplay.computed.days: ${days}, ${hours}`)
      return days
    },
  },
  watch: {
    currentDateTime(){
      console.log(`TimeDisplay.watch.currentDateTime`)
      this.emitChangeEvent(this.currentDateTime)
    }
  },
  data() {
    return {
      initialDateTime: this.time.clone(),
      currentDateTime: this.time.clone(),
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
