import * as d3 from "d3"
/**
 * custom tooltip for use in polar plots. This isn't very customizable,
 * but it correctly calculates the position of the tooltip in relation to
 * circles in polar plots.
 */
class D3ToolTip {
    constructor(options){
        options = this.processOptions(options)
        this.options = options
        this.node = d3.select(document.createElement('div'))
        this.node
            .style('position', 'absolute')
            .style('top', 0)
            .style('opacity', 0.0)
        	.style('pointer-events', 'none')
            .style('box-sizing', 'border-box')
            .attr("class", options.class)
        this.currentData = null
        this.hidden = true
        document.body.appendChild(this.node.node())
    }

    show(d, i, node){
        this.hidden = false
        this.currentData = d
        this.node.html(d.name)
        var thisNode = node[i]
        var bounds = thisNode.getBoundingClientRect()
        var scrollTop  = document.documentElement.scrollTop || document.body.scrollTop
        var scrollLeft = document.documentElement.scrollLeft || document.body.scrollLeft
        var widthOffset = parseFloat(this.node.style("width"),10)*0.5
        var heightOffset = parseFloat(this.node.style("height"), 10)*1.1
        var nodeHeight = bounds.height
        var nodeWidth = bounds.width
        this.node.style("opacity", 0.8)
        this.node.style("top",`${bounds.top + scrollTop + this.options.offsets.top - heightOffset}px`)
        this.node.style("left",`${bounds.left + scrollLeft + this.options.offsets.left - widthOffset + nodeWidth/2}px`)
    }

    hide(d, i, node){
        this.hidden = true
        var thisNode = node[i]
        this.node.style("opacity", 0.0)
    }

    processOptions(options){
        if (options === undefined){
            options = {}
        }
        options = Object.assign({
            class: "d3-tip",
            offsets: {
                top: 0.0,
                left: 0.0
            }
        }, options)
        return options
    }
}

export default D3ToolTip
