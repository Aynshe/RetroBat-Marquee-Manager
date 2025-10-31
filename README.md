<img src="https://github.com/Nelfe80/RetroBat-Marquee-Manager/blob/master/dist/images/logo.png" style="width:100%;">
<h1>RetroBat Marquee Manager (V4.0 - Rust Edition)</h1>
<h2>A Dynamic Marquees for RetroBat with Svg support / Auto-resizing / Dynamic Scraping / RetroAchievements (WIP)</h2>
<p>This project enables <b>dynamic display of marquees</b> on a secondary topper screen for RetroBat users on Windows 8+, utilizing custom scripts to manage the display based on user interactions. This new version has been completely rewritten in Rust for improved performance and reliability.</p>
<p>Thanks to Aynshe, Bob Morane, xReppa and Retrobat's community testers. </p>
<p>
    <a href="https://www.youtube.com/watch?v=7LwR_cwa0Cg" target="_blank">
        <img src="https://i.ytimg.com/vi/7LwR_cwa0Cg/hqdefault.jpg" alt="Lien vers la vidéo YouTube">
    </a>
    <a href="https://www.youtube.com/watch?v=AFS7f5RKJZo" target="_blank">
        <img src="https://i.ytimg.com/vi/AFS7f5RKJZo/hqdefault.jpg" alt="Lien vers la vidéo YouTube">
    </a>
</p>
<h2>Install</h2>
<p>
To install the Marquee Manager in RetroBat, follow these steps:<br>
- Access the Main Menu: Start by launching RetroBat. Once you're on the main screen, navigate to the Main Menu.<br>
- Updates and Downloads: In the Main Menu, look for the Updates and Downloads section. Select it to proceed.<br>
- Download Content: Within the Updates and Downloads menu, find and select the Download Content option.<br>
- Media Tab: After selecting Download Content, switch to the Medias & Tools tab.<br>
- Select Marquee Manager: In the Media & Tools tab, you will see a list of media options available for download. Look for and select Marquee Manager from the list to install it.<br>
- Launch start.bat in the RetroBat/plugins/marqueemanager/ folder.<br>
- Use BatGui for Configuration: Once the Marquee Manager is installed, you can use BatGui to further configure the settings. This includes selecting which screen to target for the marquee display and deciding whether to activate any additional modules.<br>
</p>
</p>
<h2>Building from Source (for Developers)</h2>
<p>
This project is now written in Rust. To build the executables from source, you will need to have the Rust toolchain installed, as well as the `mingw-w64` toolchain for cross-compiling to Windows.
</p>
<p>
The Rust code is organized into a monorepo in the `rust` directory. Each sub-directory is a separate Cargo project:
<ul>
    <li>`marquee_manager_rs`: The core application that replaces `ESEvents.py`.</li>
    <li>`retroachievements_rs`: The RetroAchievements module that replaces `ESRetroAchievements.py`.</li>
    <li>`utility_rs`: A multi-functional utility that replaces the remaining Python scripts.</li>
</ul>
</p>
<p>
To build all the projects, navigate to each directory and run the following command:
<code>cargo build --release --target=x86_64-pc-windows-gnu</code>
</p>
<p>
The final executables will be located in the `target/x86_64-pc-windows-gnu/release` directory of each project.
</p>

<h2>Configuring config.ini File if needed</h2>
<p>
Configure config.ini ( in the folder /RetroBat/plugins/MarqueeManager/ ) to specify paths for your own marquees and other key settings like accepted formats, MPV path and ImageMagick path, etc. This file is crucial for the marquee system to function properly. (MarqueeRetroAchievements = true in config.ini file to activate RetroAchievements or MarqueeAutoScraping = true to scrap banners...)
</p>
<p>
... (The rest of the README remains the same) ...
</p>
