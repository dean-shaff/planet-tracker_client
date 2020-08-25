function Util(){
  this.radToDegree = function(val){
    if (typeof val !== Number){
      val = parseFloat(val, 10)
    }
    return (180./Math.PI)*val
  }

  this.isMobile = function() {
    if (window.innerWidth > 768) {
      return false
    }
    return true
  }
}
const util = new Util()
export default util
