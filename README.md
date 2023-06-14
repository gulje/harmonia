# harmonia
harmonia, organize your complex Spotify playlists with a single script.

**CAUTION:** Since `harmonia` is in its very early stages, keep in mind that you may need a backup of the playlists while using it!

## Overview
Before reading:
- **Playlists**
    - `[playlist]`: Represents a single playlist and should be written as `playlist [playlist]`
    - `[playlists]`: It represents a list of playlists (`playlist1, playlist2, playlist3, ...`) and can also be replaced by `selected playlists`.
    - `[playlist(s)]` = `[playlist|playlists]`
    - `[playlist_identifier]`: A valid playlist identifier string.
- **Tracks**
    - `[track]`: Represents a single track and should be written as `track [playlist]`
    - `[tracks]`: Same as `[playlists]`.
    - `[track(s)]`: `[track|tracks]`
- **Albums**
    - `[album]`: Represents a single album and should be written as `album [album]`
    - `[albums]`: Same as `[playlists]`.
    - `[album(s)]`: `[album|albums]`.

### Selection
```toml
select tracks in [playlist(s)] with [condition(s)]
select playlists with [condition(s)]
select albums in [ ... with [condition(s)]
```

### Custom Implementations
```toml
define [sorting|duplication|renaming|exporting] implementation [custom_implementation] {
    # Custom implementation logic
}
```

### Playlists
```toml
# Creating and Merging
create playlist [playlist_identifier] with [tracks]
merge [playlists] into [playlist|playlist_identifier]

# Deleting
delete [playlist(s)]

# Renaming
rename [playlist(s)] to [playlist_identifier(s)]
rename [playlist(s)] with [custom_implementation]

# Adding, Removing, Shuffling
add [track(s)] to [playlist(s)]
remove [track(s)] from [playlist(s)]
shuffle [playlist(s)]

# Exporting and Restoring
export [playlist] into [export_file]
export [playlist(s)] with [custom_implementation]

# Sorting and Shuffling
shuffle [playlist(s)]
sort [playlist] by track [name|duration|popularity|album] [asc|dec]
sort [playlist] by artist [name] [asc|dec]
sort [playlist] with [custom_implementation]

# Duplicate Detection
detect and [select|remove|output] duplicates in [playlist(s)] with [name|...]
detect duplicates in [playlist(s)] with [custom_implementation]

# Saving the changes made to the playlist(s)
save [playlist(s)]
```

### Functions
```toml
function [function_name]([parameter(s)]) {
    # Function body
}

# Calling custom functions
call [function_name]([parameter(s)])
```

### Track Tags
```toml
# Adding and Removing Tags
add tag [tag_name] to [track(s)]
remove tag [tag_name] to [track(s)]

# Renaming Tags
rename tag [tag(s)] to [tag_name(s)]
rename tag [tag(s)] with [custom_implementation]
```

### Invoking Shell Commands
```toml
shell [command]
```