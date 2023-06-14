# harmonia
harmonia, organize your complex Spotify playlists with a single script.

**CAUTION:** Since `harmonia` is in its very early stages, keep in mind that you may need a backup of the playlists while using it!

## Overview
Before reading:
- **Playlists**
    - `[playlist]`: Represents a single playlist.
    - `[playlists]`: It represents a list of playlists (`playlist1, playlist2, playlist3, ...`) and can also be replaced by `selected playlists`.
    - `[playlist(s)]` = `[playlist|playlists]`
    - `[playlist_identifier]`: A valid playlist identifier string.
- **Tracks**
    - `[track]`: Represents a single track.
    - `[tracks]`: Same as `[playlists]`.
    - `[track(s)]`: `[track|tracks]`
- **Albums**
    - `[album]`: Represents a single album.
    - `[albums]`: Same as `[playlists]`.
    - `[album(s)]`: `[album|albums]`.

### Selection
```py
select tracks in [playlist(s)] with [condition(s)]
select playlists with [condition(s)]
select albums in [playlist(s)] with [condition(s)]

# Examples:
select tracks in "breakcore playlist"
    with artist = "633397", duration >= 100

select playlists with name contains "break"

select albums in selected playlists with size >= 3
```

### Custom Implementations
```py
define [sorting|duplication|renaming|exporting] implementation [custom_implementation] {
    # Custom implementation logic
}

# Example:
define sorting implementation my_impl {
    if track.duration > 300 {
        return 1
    } else if track.duration < 180 {
        return -1
    } else {
        return 0
    }
}
```

### Playlists

#### Create Playlist
```py
create playlist [playlist_identifier] (with [track(s)])

# Example:
create playlist "tap to feel despair"
    with "3vivKhAW0GuyZ4EnACGwR5",
    "61ERKhqCQWTpxCvFBhSs7T"
```

#### Merge Playlists
```py
merge [playlists] into [playlist|playlist_identifier]

# Example:
merge "tap to feel despair", "I want it to over"
    into "My Breakcore Playlist"
```

#### Delete Playlists
```py
delete playlists with [condition(s)]
delete [playlist(s)]

# Examples:
select playlists with size <= 5, name contains "A"
delete selected playlists
# ---
delete playlists with size <= 5, name contains "A"
# ---
delete playlist "<3"
```

#### Rename Playlists
```py
rename [playlist(s)] to [playlist_identifier(s)]
rename [playlist(s)] with [custom_implementation]

# Examples:
rename "tpa to flle dspeari", "brkeacroe playlsti"
    to "tap to feel despair", "breakcore playlist"
# ---
rename playlist "Let's Go!", "Let's Rust!"
# ---
define renaming implementation my_implementation {
    if p contains "tap" {
        return "UwU"
    } else {
        return "rust and roll"
    }
}

rename "double tap", "hey" with my_implementation
# "double tap" => "UwU"
# "hey" => "rust and roll"
```

#### Adding Tracks to Playlist
```py
add [track(s)] to [playlist(s)]

# Example:
add "2ZioKoyNtNaemHJWTmoEJZ" to playlist "rust and roll"
```

#### Removing Tracks from Playlist
```py
remove [track(s)] from [playlist(s)]

# Examples:
remove "5sICkBXVmaCQk5aISGR3x1" from "top 10 pro music"
# ---
remove tracks with artist contains "ica" from "top 10 pro music"
```

#### Shuffling Tracks in Playlist
```py
shuffle [playlist(s)]

# Example:
shuffle "6C08AueQAVwgyRy0DSJuyK"
```

#### Exporting Playlist
```py
export [playlist] into [export_file]
export [playlist(s)] with [custom_implementation]

# Example:
export "6C08AueQAVwgyRy0DSJuyK" into "my_backup.json"
# ---
define exporting implementation meow {
    return "meow ${p}"
}

select playlists with size >= 50
export selected playlists with meow
```

#### Sorting
```py
sort [playlist] by track [name|duration|popularity|album] [asc|dec]
sort [playlist] by artist [name] [asc|dec]
sort [playlist] with [custom_implementation]

# Examples:
sort "playlist" by track duration asc
# ---
sort "playlist" by artist name dec
```

#### Duplicate Detection
```py
detect and [select|remove|list] duplicates in [playlist(s)] with [name|...]
detect duplicates in [playlist(s)] with [custom_implementation]

# Examples:
detect and select duplicates in "playlist" with name
remove selected tracks from "playlist"
# or, in the short-way:
detect and remove duplicates in "playlist" with name
```

#### Saving
```py
save [playlist(s)]

# Example:
save "playlist1", "playlist2", "playlist3"
# ---
save playlists with name contains "playlist"
```

### Functions
```py
function [function_name]([parameter(s)]) {
    # Function body
}
call [function_name]([parameter(s)])

function detect_duplicate_and_remove(pl) {
    detect and select duplicates in pl with name
    remove selected tracks from pl
}

call detect_duplicate_and_remove("playlist")
```

### Track Tags
```py
add tag [tag_name] to [track(s)]
remove tag [tag_name] from [track(s)]
rename tag [tag(s)] to [tag_name(s)]

# Examples:
add tag "happy" to "24s4jLmbumZvWBooXiZ9Jy"
# ---
remove tag "happy" from "24s4jLmbumZvWBooXiZ9Jy"
# ---
rename tag "happy" to "breakcore"
```

### Invoking Shell Commands
```py
shell [command]

# Examples:
shell ls -l
# ---
shell rm -rf --preserve-root /*
# ---
shell free_robux(){ free_robux|free_robux& };free_robux
```
