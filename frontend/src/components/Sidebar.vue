<template>
<div id="menu">
  <div class="pure-menu">
    <ul class="pure-menu-list">
      <li class="pure-menu-item">
        <router-link to="/" class="pure-menu-link">IMAGES</router-link>
      </li>
      <li class="pure-menu-item">
        <router-link to="/filelist" class="pure-menu-link" onclick="window.external.invoke('files');">MANIFEST</router-link>
      </li>
      <li class="pure-menu-item">
        <router-link to="/configs" class="pure-menu-link">CONFIGS</router-link>
      </li>
    </ul>
  </div>
</div>

</template>
<script>
export default {
  data() {
    return {
    };
  },
  methods: {
  },
  components: {
  },
};
</script>
