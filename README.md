# jsan
The **J**SON **S**wiss **A**rmy k**N**ife. Convert, subset, repurpose, and otherwise extract information in JSON streams/files from the command line

`jsan` provides extraction of data from JSON streams (from file or standard input) and returns (to file or standard output) the result in user-customizable format. A common application is to convert JSON-encoded data into comma separated values (CSV) format.  Conversion is fast and does not consume memory, allowing for conversion of arbitrarily large (or endlessly streaming) JSON files. Data fields can optionally be filtered during conversion so that only desired fields are retained in the resulting output. 

Because `jsan` can also write to standard output, it can also serve as a useful exploration tool for quickly exploring JSON data (e.g., pulling user names across all entries). Output can be piped to other common command line tools (e.g., awk) as in traditional unix-like workflows.

## License

Copyright (C) 2019 Nicholas M. Van Horn

Author: Nicholas M. Van Horn <nvanhorn@nicholasvanhorn.com>

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
