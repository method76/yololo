import ImageList from './views/ImageList.vue'
import FileList from './views/FileList.vue'

const routes = [
    { path: '/', component: ImageList },
    { path: '/filelist', component: FileList },
];

export default routes;