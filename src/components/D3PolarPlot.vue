<template>
  <div></div>
</template>

<script>
import Vue from "vue"
import * as d3 from "d3"

import D3ToolTip from "./D3ToolTip.js"

export default {
  props: {
    circles: {type:Array, default:()=>{return []}},
    circleOptions: {type:Object, default:()=>{return {}}},
    height: {type:Number, default:300},
    width: {type:Number, default:300},
    options: {type:Object, default:()=>{return {ticks: 6}}},
    tooltipTarget: {type: Array, default: () => null},
  },
  mounted: function(){
    var radius = 0.90*(Math.min(this.elWidth, this.elHeight)/2)
    var radiusScale = d3.scalePow()
      .exponent(1.5)
      .domain([90, 0])
      .range([0, radius])
    // var radiusScale = d3.scaleLinear()
    //   .domain([90, 0])
    //   .range([0, radius])
    // //
    // var radiusScale = (d) => {
    //   let xReflect = - 1
    //   let val = Math.abs(Math.pow(xReflect * ((d/90) - 1), 2.0))*radius
    //   if (val < 0) {
    //     val = 0.0
    //   } else if (val > radius) {
    //     val = radius
    //   }
    //   return val
    // }

    var plot = this.createSVG(this.$el)
    var radialPlot = this.createRadialPlot(plot, radiusScale)
    var angularPlot = this.createAngularPlot(plot, radius)
    var outerCircle = this.createOuterCircle(plot)
    var [outerArc, largeArc] = this.createOuterArc(plot,radius, 15)
    this.plot = plot
    this.scale = radiusScale
    this.tooltip = new D3ToolTip({class: "d3-tip"})
    this.currentElem = null

    this.axisLabelFontSize = "10px"

  },
  methods: {
    createSVG(mount){
      var svg = d3.select(mount).append("svg")
        .attr("width", this.elWidth)
        .attr("height", this.elHeight)
        .append("g")
        .attr("transform", `translate(${this.elWidth/2},${this.elHeight/2})`)
      return svg.append("g")
    },
    createRadialPlot(plotGroup, scale){
      console.log(`D3PolarPlot.createRadialPlot: ticks=${this.options.ticks}`)
      // let ticks = scale.ticks(this.options.ticks)
      // if (ticks[0] == 90) {
      //   ticks = ticks.slice(1)
      // }
      let ticks = [75, 60, 45, 30, 15, 0]
      // let ticks = [80, 60, 40, 20, 0]

      var plotRadial = plotGroup.append("g")
        .attr("class", "r axis")
        .selectAll("g")
        .data(ticks)
        .enter().append("g")

      plotRadial.append("circle")
        .style("stroke", "rgba(0,0,0,0.4)")
        .style('fill', 'none')
        .attr("r", scale)

      plotRadial.append("text")
        .attr("y", (d)=>{ return - scale(d) - 4; })
        // .attr("y", (d)=>{ return + scale(90 - d) - 4; })
        .attr("transform", "rotate(15)")
        .style("font-size", "12px")
        .style("text-anchor", "middle")
        .text((d)=>{ return d+ "°"; })
      return plotRadial
    },
    createAngularPlot: function(plotGroup, radius){
      var plotAngular = plotGroup.append("g")
        .attr("class", "a axis")
        .selectAll("g")
        .data(d3.range(0, 360, 30))
          .enter().append("g")
          .attr("transform", (d)=>{return `rotate(${d-90})`});

      plotAngular.append("line")
        .attr("x2", radius)
        .style("stroke", "rgba(0,0,0,0.4)")
        .style('fill', 'none')

      let fontSize = this.axisLabelFontSize
      plotAngular.append("text")
        .attr("x", radius+6)
        .attr("dy", ".35em")
        .style("font-size", "12px")
        .style("text-anchor", (d)=>{
          if (d < 360 && d > 180){
            return "end" ;
          }else{
            return null ;
          }
        })
        .attr("transform", (d)=>{
          if (d < 360 && d > 180){
            return `rotate(180 ${radius+6}, 0)`
          }else {
            return null ;
          }
        })
        .text((d)=>{ return d + "°"; })
      return plotAngular
    },

    createOuterCircle: function(plotGroup, radius){
      var outerCircle = plotGroup.append('circle')
        .style("stroke", "rgba(0,0,0,0.4)")
        .style('fill', 'none')
        .attr('r', radius)
      return outerCircle
    },

    createOuterArc: function(plotGroup, radius, tolerance){
      var largeArc = d3.arc()
        .innerRadius(radius - tolerance)
        .outerRadius(radius + tolerance)
        .startAngle(0)
        .endAngle(2*Math.PI)

      var outerArc = plotGroup.append('path')
        .attr('d', largeArc)
        .attr('id', 'horizon')
        .style("stroke", "none")
        .style("fill", "rgba(0,0,0,0.0)")
      return [outerArc, largeArc]
    },
    processPlotOptions: function(kwargs){
      if (kwargs == undefined){
        kwargs = {}
      }
      kwargs = Object.assign({
        r: 10,
        stroke: "rgba(0,0,0,0.0)",
        fill: "rgba(0,0,0,0.4)",
        opacity: 0.8,
        mouseover: (d, i, node) => {
          if (this.tooltip.currentData == null){
            this.tooltip.show(d, i, node)
          }
          this.tooltip.currentData = d
          // if (this.tooltip.currentData != this.tooltipClick.currentData){
          this.tooltip.show(d, i, node)
          // }
          this.$emit("circle-mouseover",d)
        },
        mouseout: (d, i, node)=>{
          this.tooltip.hide(d, i, node)
          this.$emit("circle-mouseout", d)
        },
        // click: (d, i, node)=>{
        //   this.tooltip.hide(d, i, node)
        //   var beforeData = this.tooltipClick.currentData
        //   this.tooltipClick.currentData = d
        //   if (this.tooltipClick.currentData == beforeData){
        //     if (! this.tooltipClick.hidden){
        //       this.tooltipClick.hide(d, i, node)
        //     }else{
        //       this.tooltipClick.show(d,i,node)
        //     }
        //   }else{
        //     this.tooltipClick.show(d,i,node)
        //   }
        //   this.$emit("circle-click",d)
        // },
        dblclick: (d,i,node)=>{this.$emit("circle-dbclick",d,i,node)},
        class: "scatter"
      },kwargs)
      return kwargs
    },
    clearCircles: function(){
      this.plot.selectAll("circle.scatter").remove()
    },
    hideCircles: function(type){
      // console.log(`D3PolarPlot.hideCircles: type: ${type}`)
      this.plot.selectAll(`circle.scatter.${type}`)
        .attr("r", 0.0)
    },
    showCircles: function(type){
      // console.log(`D3PolarPlot.showCircles: type: ${type}`)
      this.plot.selectAll(`circle.scatter.${type}`)
        .attr("r", this.circleOptions[type].r)
    },
    updateCircles: function(data){
      var kwargs = this.processPlotOptions()
      // console.log(kwargs["click"])
      var u = this.plot.selectAll("circles")
        .data(data)
      var getOption = (param)=>{
        return (d)=>{
          var optionVal = kwargs[param]
          if ("name" in d){
            if (d.name in this.circleOptions){
              optionVal = this.circleOptions[d.name][param]
            }
          }
          if (optionVal != undefined){
            if (optionVal.constructor === Function){
              optionVal = optionVal(d)
              // if (param === "r"){
              //   console.log(optionVal)
              // }
            }
          }
          return optionVal
        }
      }
      // console.log(`D3PolarPlot.updateCircles: ${getOption('click')}`)
      u.enter().append("circle")
        .merge(u)
        .attr("id",(d) => d.name)
        .attr("class", getOption("class"))
        .attr("cx", (d) => {return this.scale(d.el)})
        .attr("transform", (d) => {return `rotate(${d.az-90})`})
        .attr('r', getOption("r"))
        .style("stroke",getOption("stroke"))
        .style("fill",getOption("fill"))
        .style("opacity",getOption("opacity"))
        .on("mouseover",kwargs.mouseover)
        .on("mouseout",kwargs.mouseout)
        // .on("click",kwargs.click)
        .on("dblclick",kwargs.dblclick)
    },
    filterNode: function(filterFn){
      var filtered = d3.selectAll("circle.scatter").filter(filterFn)
      return filtered
    }
  },
  computed:{
    key: function(){
      if (this.plot !== null){
        this.clearCircles()
        this.updateCircles(this.circles)
      }
    },
    elWidth: function () {
      return 0.95*this.width
    },
    elHeight: function () {
      return 0.95*this.height
    }
  },
  watch:{
    circles(data){
      this.clearCircles()
      this.updateCircles(data)
    },
    tooltipTarget(newTooltipTarget){
      console.log(`D3PolarPlot.tooltipTarget: ${newTooltipTarget}`)
      if (newTooltipTarget != null) {
        let elem = d3.select(`#${newTooltipTarget[1]}`)
        elem.dispatch("mouseover")
        this.currentElem = elem
      } else {
        if (this.currentElem != null) {
          this.currentElem.dispatch("mouseout")
        }
      }
    }
  },
  data: function(){
    return {
      plot: null,
      scale: null,
    }
  }
}

</script>

<style scoped>

</style>
