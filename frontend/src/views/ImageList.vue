<template>

  <div id="layout">

    <div id="main">
      <div id="id-contents" class="content pure-g">
        <div class="pure-u-1-12" v-for="item in items" v-bind:key="item.url">
          <div @click="imagePopup(item.url)" v-bind:style="{ backgroundImage: 'url(file://' + item.url + ')' }">
            <img class="ic-checked" src="/img/ic-checked.png" v-if="item.labeled">
          </div>
        </div>
        <div class="pure-u-1-1">
          <div onclick="window.external.invoke('open')">PLEASE SET IMAGE DIRECTORY</div>
        </div>
      </div>
    </div>

    <!-- lightbox container hidden with CSS -->
    <div class="lightbox" v-show="showingPopup">
      <canvas id="span-image-detail" ref="canvas" @mousedown="startSelect" @mousemove="drawRect" @mouseup="stopSelect"></canvas>
      <div id="close-canvas" @click="closePop">X</div>
    </div>

  </div>
</template>
<script>
import 'whatwg-fetch'

export default {
  name: "image-list",
  components: {
  },
  data() {
    return {
      ctx: null,
      canvas: null,
      showingPopup: false,
      selectionMode: false,
      startPosition: {
        x: null,
        y: null
      },
      items: null
    }
  },
  methods: {
    requestImages() {
      window.fetch('/api/v1/images')
          .then((response) => {
            if(response.ok){
              return response.json();
            }
            throw new Error('Network response was not ok');
          })
          .then((json) => {
            this.items = json;
          })
          .catch(() => {
            // console.log(error);
          })
    },
    listImages() {
      /*
      let jsonArr = JSON.parse(jsonArrStr);
      let contents = "";
      if (path) { document.getElementById("item-path").innerText = path; }
      for (let i=0; jsonArr.length; i++) {
        if (!jsonArr[i]) {
          break;
        }
        let local_url = "file://" + jsonArr[i].url.replace(/\\/gi, '/');
        let labeled = jsonArr[i].labeled?'highlight':'';
        let labeled_txt = jsonArr[i].labeled?'<img class="ic-checked" src="/img/ic-checked.png">':'';
        contents += '<div class="pure-u-1-12">'
            + '<div class="' + labeled + '" style="background-image: url(\'' + local_url + '\');" onclick="detail_image(\'' + local_url + '\')">'
            + labeled_txt + '</div></div>';
      }
      document.getElementById("id-contents").innerHTML = contents;
      */
    },
    imagePopup(img_url) {
      this.showingPopup = true;
      let img = new Image();
      let local = this;
      img.onload = function(){
        local.canvas.width = img.naturalWidth;
        local.canvas.height = img.naturalHeight;
        local.ctx.drawImage(img, 0, 0, img.naturalWidth, img.naturalHeight);
        // Todo: Rect 정보 fetch 해오기
      };
      img.src = img_url;
    },

    startSelect(e) {
      this.selectionMode = true;
      this.startPosition.x = e.clientX;
      this.startPosition.y = e.clientY;
    },

    drawRect() {
      if (this.selectionMode) {
        // console.log(this.startPosition);
        // this.ctx.beginPath();
        // this.ctx.rect(
        //     this.startPosition.x,
        //     this.startPosition.y,
        //     e.clientX - this.startPosition.x,
        //     e.clientY - this.startPosition.y
        // );
        // this.ctx.closePath();
        // this.ctx.fillRect(0, 0, window.innerWidth, window.innerHeight);
        // this.ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);
        // this.ctx.strokeStyle = "#f00";
        // this.ctx.stroke();
        this.ctx.beginPath();
        this.ctx.lineWidth = "3";
        this.ctx.strokeStyle = "red";
        let imgWidth = this.canvas.width;
        let imgHeight = this.canvas.height;
        let x = imgWidth * 0.136335 - imgWidth * 0.159627 / 2;
        let y = imgHeight * 0.351939 - imgHeight * 0.240893 / 2;
        this.ctx.rect(x, y, imgWidth * 0.159627, imgHeight * 0.240893);
        this.ctx.stroke();
      }
    },
    stopSelect() {
      this.ctx.fillStyle = "#fff";
      this.selectionMode = false;
      this.startPosition.x = null;
      this.startPosition.y = null;
    },
    closePop() {
      this.showingPopup = false;
    }

  },
  mounted() {
    /*
    this.$refs.select.height = window.innerHeight;
    this.$refs.select.width = window.innerWidth;
    */
    this.canvas = this.$refs.canvas;
    this.ctx = this.canvas.getContext("2d");
    this.requestImages();
  }

};
</script>
