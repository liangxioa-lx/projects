import { TrayIcon } from '@tauri-apps/api/tray';
import { Menu } from '@tauri-apps/api/menu';
import {getAllWindows, getCurrentWindow} from "@tauri-apps/api/window";
import {exit, relaunch} from '@tauri-apps/plugin-process';



const menu = await Menu.new({
    items: [
        {
            id: 'open',
            text: '打开窗口',
            action: async () => {
                const win = await getAllWindows();
                if (win.length > 0) {
                    win[0].show();
                }
            },
        },
        {
            id: 'relaunch',
            text: '重新启动',
            action: () => {
               relaunch();
            },
        },
        {
            id: 'quit',
            text: '退出',
            action: () => {
               exit(1);
            },
        },
    ],
});

const tray = TrayIcon.new({
    tooltip: '文件管理工具',
    icon : "icons/32x32.png",
    menu,
    menuOnLeftClick: false,
    action: async (event) => {
        // 左键点击事件
        if (event.type === 'Click' && event.button === "Left" && event.buttonState === 'Down') {
            console.log('单击事件');
            const win = await getAllWindows();
            if (win.length > 0) {
                win[0].show();
            }
        }
    }
})