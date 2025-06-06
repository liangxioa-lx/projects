import Database from '@tauri-apps/plugin-sql';

const db = await Database.load('sqlite:projects.db');

export default db;