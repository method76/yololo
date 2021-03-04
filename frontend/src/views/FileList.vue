<template>

  <div id="layout">

    <div id="main">
      <div v-for="item in items" v-bind:key="item.url">
        <p>{{ item.url }}</p>
      </div>
    </div>

  </div>
</template>
<script>
export default {
  components: {
  },
  data() {
    return {
      items: null
    }
  },
  methods: {
    requestTrainTxt() {
      window.fetch('/api/v1/files')
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
    }
  },
  mounted() {
    this.requestTrainTxt();
  }
};
</script>
