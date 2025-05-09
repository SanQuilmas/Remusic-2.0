import { MidiBox } from "../MidiBox/MidiBox"
import { SheetMusicBox } from "../SheetMusicBox/SheetMusicBox"

export const MainMusicContainer = () => {
    return (
        <div>
            <h1> Title </h1>
            <div><a href="#"> Download MusicXml </a></div>
            <div>
                <SheetMusicBox />
            </div>

            <div><a href="#"> Download MIDI </a></div>
            
            <div>
                <MidiBox />
            </div>

        </div>
    )
}