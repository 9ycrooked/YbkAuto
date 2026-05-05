import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { ask, message } from '@tauri-apps/plugin-dialog';

export async function checkUpdate() {
  try {
    const update = await check();

    if (update) {
      const yes = await ask(
        `发现新版本 ${update.version}，是否立即更新？`,
        { title: '更新提示', okLabel: '更新', cancelLabel: '稍后' }
      );

      if (yes) {
        await update.downloadAndInstall();
        await relaunch();
      }
    } else {
      await message(
        '当前已是最新版本',
        { title: '检查更新', okLabel: '确定' }
      );
    }
  } catch (error) {
    console.error('检查更新失败:', error);
  }
}