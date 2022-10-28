[![Rust build status](https://img.shields.io/github/workflow/status/travisbrown/twitter-watch/rust-ci.svg?label=rust)](https://github.com/travisbrown/twitter-watch/actions)

# Twitter reports

* [Overview](#overview)
* [Top {{ top_suspensions | length }} suspensions](#top-{{ top_suspensions | length }}-suspensions)
* [Daily reports](#daily-reports)
* [License](#license)

## About

This project tracks far-right and far-right-adjacent accounts on Twitter.

Each daily report includes three tables:

* A list of _tracked_ accounts identified as suspended on that day, sorted by their centrality to far-right networks.
* A list of _tracked_ accounts identified as having changed their screen name on that day.
* A list of _untracked_ accounts identified as suspended on that day, sorted by follower count.

In this context a _tracked_ account is one that is identified as having connections to far-right networks on Twitter.
The presence of an account on this list does not indicate that it is identified as supporting far-right causes
or spreading far-right content, just that it has connections to far-right networks.

The third table only includes suspended _untracked_ accounts with more than {{ untracked_suspensions_limit }} followers.
These are often bots or spam accounts.

Suspension (and suspension reversal) dates indicate when the change in account status was detected,
which in some cases may be up to several days after the change occurred.

![Chart of total daily suspension counts](charts/suspensions.svg)
![Chart of total daily screen name change counts](charts/screen-names.svg)

## Overview

* Total number of suspensions detected: {{ stats.total_suspensions_count }}
* Total number of suspension reversals detected: {{ stats.total_reversals_count }}
* Mean number of followers for suspended accounts: {{ stats.other[0].1.1 | round(precision = 2) }}
* Median number of followers for suspended accounts: {{ stats.other[0].1.0.1 | round(precision = 2) }}
* Mean age of suspended accounts (days): {{ stats.other[1].1.1 | round(precision = 2) }}
* Median age of suspended accounts (days): {{ stats.other[1].1.0.1 | round(precision = 2) }}
* Total number of verified accounts suspended: {{ stats.total_verified_suspended_count }}
* Total number of protected accounts suspended: {{ stats.total_protected_suspended_count }}
* Total number of suspensions for accounts previously withheld in specific countries: {{ stats.total_withheld_suspended_count }}
* Total number of screen name changes detected: {{ stats.total_screen_name_changes_count }}

{#
<table>
    <tr>
        <th></th>
        <th align="left">P25</th>
        <th align="left">Median</th>
        <th align="left">P75</th>
        <th align="left">Mean</th>
    </tr>
    {%- for item in stats.other %}
        <tr>
            <td>{{ item.0 }}</td>
            <td>{{ item.1.0.0 | round(precision = 2) }}</td>
            <td>{{ item.1.0.1 | round(precision = 2) }}</td>
            <td>{{ item.1.0.2 | round(precision = 2) }}</td>
            <td>{{ item.1.1 | round(precision = 2) }}</td>
        </tr>
    {%- endfor -%}
</table>
#}

## Top {{ top_suspensions | length }} suspensions

Current list of suspensions of accounts most central to far-right networks since the creation of this project.

<table>
    <tr>
        <th></th>
        <th align="left">Screen name</th>
        <th align="left">Created</th>
        <th align="left">Suspended</th>
        <th align="left">Followers</th>
        <th align="left">Ranking</th></tr>
    </tr>
    {%- for item in top_suspensions %}
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id={{ item.0.record.user_id }}">
                <img src="{{ item.0.image_url }}" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/{{ item.0.record.screen_name }}">{{ item.0.record.screen_name }}</a>
                {%- if item.0.other_screen_name_count == 0 -%}
                {%- else -%}
                    &nbsp;(<a href="https://api.memory.lol/v1/tw/id/{{ item.0.record.user_id }}">
                        {{- item.0.other_screen_name_count }} other{{ item.0.other_screen_name_count | pluralize -}}
                    </a>)&nbsp;
                {%- endif -%}
            </td>
            <td>{{ item.0.record.created_at | date(format="%Y-%m-%d") }}</td>
            <td>{{ item.1 | date(format="%Y-%m-%d") }}</td>
            <td>{{ item.0.record.followers_count }}</td>
            <td>{{ item.0.ranking }}</td>
        </tr>
    {%- endfor -%}
</table>

## Daily reports

<table>
    <tr>
        <th align="left">Date</th>
        <th align="left">Total suspensions</th>
        <th align="left">Tracked suspensions</th>
        <th align="left">Tracked screen name changes</th>
    </tr>
    {%- for item in date_stats %}
        <tr>
            <td>
                <a href="reports/{{ item.date | date(format='%Y-%m-%d') }}/">
                    {{- item.date | date(format="%e %B %Y") -}}
                </a>
            </td>
            <td>{{ item.total_suspensions_count }}</td>
            <td>{{ item.tracked_suspensions_count }}</td>
            <td>{{ item.tracked_screen_name_changes_count }}</td>
        </tr>
    {%- endfor -%}
</table>

## License

The software in this repository (currently only report formatting code) is published under the [Anti-Capitalist Software License][acsl] (v. 1.4).

[acsl]: https://anticapitalist.software/
