(function(t){function e(e){for(var i,a,o=e[0],u=e[1],l=e[2],p=0,d=[];p<o.length;p++)a=o[p],Object.prototype.hasOwnProperty.call(s,a)&&s[a]&&d.push(s[a][0]),s[a]=0;for(i in u)Object.prototype.hasOwnProperty.call(u,i)&&(t[i]=u[i]);c&&c(e);while(d.length)d.shift()();return r.push.apply(r,l||[]),n()}function n(){for(var t,e=0;e<r.length;e++){for(var n=r[e],i=!0,o=1;o<n.length;o++){var u=n[o];0!==s[u]&&(i=!1)}i&&(r.splice(e--,1),t=a(a.s=n[0]))}return t}var i={},s={app:0},r=[];function a(e){if(i[e])return i[e].exports;var n=i[e]={i:e,l:!1,exports:{}};return t[e].call(n.exports,n,n.exports,a),n.l=!0,n.exports}a.m=t,a.c=i,a.d=function(t,e,n){a.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:n})},a.r=function(t){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},a.t=function(t,e){if(1&e&&(t=a(t)),8&e)return t;if(4&e&&"object"===typeof t&&t&&t.__esModule)return t;var n=Object.create(null);if(a.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var i in t)a.d(n,i,function(e){return t[e]}.bind(null,i));return n},a.n=function(t){var e=t&&t.__esModule?function(){return t["default"]}:function(){return t};return a.d(e,"a",e),e},a.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},a.p="/";var o=window["webpackJsonp"]=window["webpackJsonp"]||[],u=o.push.bind(o);o.push=e,o=o.slice();for(var l=0;l<o.length;l++)e(o[l]);var c=u;r.push([0,"chunk-vendors"]),n()})({0:function(t,e,n){t.exports=n("56d7")},"56d7":function(t,e,n){"use strict";n.r(e);n("e623"),n("e379"),n("5dc8"),n("37e1");var i=n("2b0e"),s=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{attrs:{id:"app"}},[t._m(0),n("router-view"),n("sidebar-component")],1)},r=[function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("div",{staticClass:"header"},[i("div",{staticClass:"pure-menu pure-menu-horizontal"},[i("div",{staticStyle:{float:"left","padding-left":"12px"}},[i("img",{staticStyle:{height:"38px"},attrs:{src:n("f770")}})]),i("div",{staticStyle:{float:"right","padding-right":"12px","padding-top":"4px"}},[t._v(" Path "),i("span",{attrs:{id:"item-path"}}),i("button",{staticClass:"button-small pure-button button-warning",attrs:{onclick:"window.external.invoke('open')"}},[t._v("Change Folder")]),t._v(" / Total "),i("span",{staticClass:"item-count"}),t._v(" / Current Page "),i("span",{staticClass:"item-count"}),t._v(" / Marked "),i("span",{staticClass:"item-count"}),t._v(" / UnMarked "),i("span",{staticClass:"item-count"})])])])}],a=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{attrs:{id:"menu"}},[n("div",{staticClass:"pure-menu"},[n("ul",{staticClass:"pure-menu-list"},[n("li",{staticClass:"pure-menu-item"},[n("router-link",{staticClass:"pure-menu-link",attrs:{to:"/"}},[t._v("IMAGES")])],1),n("li",{staticClass:"pure-menu-item"},[n("router-link",{staticClass:"pure-menu-link",attrs:{to:"/filelist",onclick:"window.external.invoke('files');"}},[t._v("MANIFEST")])],1),n("li",{staticClass:"pure-menu-item"},[n("router-link",{staticClass:"pure-menu-link",attrs:{to:"/configs"}},[t._v("CONFIGS")])],1)])])])},o=[],u={data:function(){return{}},methods:{},components:{}},l=u,c=n("2877"),p=Object(c["a"])(l,a,o,!1,null,null,null),d=p.exports,f={name:"dashboard-page",components:{SidebarComponent:d},data:function(){return{}}},h=f,v=Object(c["a"])(h,s,r,!1,null,null,null),m=v.exports,g=n("8c4f"),w=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{attrs:{id:"layout"}},[n("div",{attrs:{id:"main"}},[n("div",{staticClass:"content pure-g",attrs:{id:"id-contents"}},[t._l(t.items,(function(e){return n("div",{key:e.url,staticClass:"pure-u-1-12"},[n("div",{style:{backgroundImage:"url(file://"+e.url+")"},on:{click:function(n){return t.imagePopup(e.url)}}},[e.labeled?n("img",{staticClass:"ic-checked",attrs:{src:"/img/ic-checked.png"}}):t._e()])])})),t._m(0)],2)]),n("div",{directives:[{name:"show",rawName:"v-show",value:t.showingPopup,expression:"showingPopup"}],staticClass:"lightbox"},[n("canvas",{ref:"canvas",attrs:{id:"span-image-detail"},on:{mousedown:t.startSelect,mousemove:t.drawRect,mouseup:t.stopSelect}}),n("div",{attrs:{id:"close-canvas"},on:{click:t.closePop}},[t._v("X")])])])},b=[function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{staticClass:"pure-u-1-1"},[n("div",{attrs:{onclick:"window.external.invoke('open')"}},[t._v("PLEASE SET IMAGE DIRECTORY")])])}],_=(n("d3b7"),n("6d93"),{name:"image-list",components:{},data:function(){return{ctx:null,canvas:null,showingPopup:!1,selectionMode:!1,startPosition:{x:null,y:null},items:null}},methods:{requestImages:function(){var t=this;window.fetch("/api/v1/images").then((function(t){if(t.ok)return t.json();throw new Error("Network response was not ok")})).then((function(e){t.items=e})).catch((function(){}))},listImages:function(){},imagePopup:function(t){this.showingPopup=!0;var e=new Image,n=this;e.onload=function(){n.canvas.width=e.naturalWidth,n.canvas.height=e.naturalHeight,n.ctx.drawImage(e,0,0,e.naturalWidth,e.naturalHeight)},e.src=t},startSelect:function(t){this.selectionMode=!0,this.startPosition.x=t.clientX,this.startPosition.y=t.clientY},drawRect:function(){if(this.selectionMode){this.ctx.beginPath(),this.ctx.lineWidth="3",this.ctx.strokeStyle="red";var t=this.canvas.width,e=this.canvas.height,n=.136335*t-.159627*t/2,i=.351939*e-.240893*e/2;this.ctx.rect(n,i,.159627*t,.240893*e),this.ctx.stroke()}},stopSelect:function(){this.ctx.fillStyle="#fff",this.selectionMode=!1,this.startPosition.x=null,this.startPosition.y=null},closePop:function(){this.showingPopup=!1}},mounted:function(){this.canvas=this.$refs.canvas,this.ctx=this.canvas.getContext("2d"),this.requestImages()}}),x=_,k=Object(c["a"])(x,w,b,!1,null,null,null),y=k.exports,C=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{attrs:{id:"layout"}},[n("div",{attrs:{id:"main"}},t._l(t.items,(function(e){return n("div",{key:e.url},[n("p",[t._v(t._s(e.url))])])})),0)])},P=[],S={components:{},data:function(){return{items:null}},methods:{requestTrainTxt:function(){var t=this;window.fetch("/api/v1/files").then((function(t){if(t.ok)return t.json();throw new Error("Network response was not ok")})).then((function(e){t.items=e})).catch((function(){}))}},mounted:function(){this.requestTrainTxt()}},O=S,j=Object(c["a"])(O,C,P,!1,null,null,null),E=j.exports,M=[{path:"/",component:y},{path:"/filelist",component:E}],I=M;i["a"].config.productionTip=!1,i["a"].use(g["a"]);var T=new g["a"]({routes:I});new i["a"]({router:T,render:function(t){return t(m)}}).$mount("#app")},f770:function(t,e,n){t.exports=n.p+"img/ic-logo.63bfd9a9.png"}});
//# sourceMappingURL=app.00a72a29.js.map