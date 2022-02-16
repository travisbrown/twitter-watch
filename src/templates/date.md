## {{ date | date(format="%e %B %Y") }}

* [Tracked suspensions ({{ tracked_suspensions | length }})](#tracked-suspensions)
* [Tracked screen name changes ({{ tracked_screen_name_changes | length }})](#tracked-screen-name-changes)
* [Untracked suspensions ({{ untracked_suspensions | length }})](#untracked-suspensions)

Please see the [project index](https://github.com/travisbrown/twitter-watch) for more information about the format of this report.

### Tracked suspensions

Found {{ tracked_suspensions | length }} suspensions for tracked accounts.
  ✔️ indicates that the account was verified and 🔒 that it was locked.

<table>
    <tr>
        <th></th>
        <th align="left">Screen name</th>
        <th align="left">Created</th>
        <th align="left">Reversed</th>
        <th align="left">Status</th>
        <th align="left">Followers</th>
        <th align="left">Ranking</th></tr>
    </tr>
    {%- for item in tracked_suspensions %}
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id={{ item.record.user_id }}">
                <img src="{{ item.image_url }}" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/{{ item.record.screen_name }}">{{ item.record.screen_name }}</a>
                {%- if item.other_screen_name_count == 0 -%}
                {%- else -%}
                    &nbsp;(<a href="https://memory.lol/tw/id/{{ item.record.user_id }}">
                        {{- item.other_screen_name_count }} other{{ item.other_screen_name_count | pluralize -}}
                    </a>)&nbsp;
                {%- endif -%}
            </td>
            <td>{{ item.record.created_at | date(format="%Y-%m-%d") }}</td>
            <td>{% if item.record.reversal %}{{ item.record.reversal | date(format="%Y-%m-%d") }}{% else %}{% endif %}</td>
            <td align="center">
                {%- if item.record.protected == 0 %}🔒{% else %}{% endif %}{% if item.record.verified == 0 -%}✔️{%- else -%}{%- endif -%}
            </td>
            <td>{{ item.record.followers_count }}</td>
            <td>{{ item.ranking }}</td>
        </tr>
    {%- endfor -%}
</table>

### Tracked screen name changes

Found {{ tracked_screen_name_changes | length }} screen name changes for tracked accounts.
{%- if tracked_screen_name_changes -%}
  ✔️ indicates that the account is verified and 🔒 that it is locked.

<table>
    <tr>
        <th></th>
        <th align="left">Previous screen name</th>
        <th align="left">New screen name</th>
        <th align="left">Status</th>
        <th align="left">Followers</th>
        <th align="left">Ranking</th></tr>
    </tr>
    {%- for item in tracked_screen_name_changes %}
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id={{ item.record.user_id }}">
                <img src="{{ item.image_url }}" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/{{ item.record.previous_screen_name }}">{{ item.record.previous_screen_name }}</a>
                {%- if item.other_screen_name_count == 0 -%}
                {%- else -%}
                    &nbsp;(<a href="https://memory.lol/tw/id/{{ item.record.user_id }}">
                        {{- item.other_screen_name_count }} other{{ item.other_screen_name_count | pluralize -}}
                    </a>)&nbsp;
                {%- endif -%}
            </td>
            <td>
                <a href="https://twitter.com/{{ item.record.new_screen_name }}">{{ item.record.new_screen_name }}</a>
            </td>
            <td align="center">
                {%- if item.record.protected == 0 %}🔒{% else %}{% endif %}{% if item.record.verified == 0 -%}✔️{%- else -%}{%- endif -%}
            </td>
            <td>{{ item.record.followers_count }}</td>
            <td>{{ item.ranking }}</td>
        </tr>
    {%- endfor -%}
</table>
{% else %}{% endif %}

### Untracked suspensions

Found {{ untracked_suspensions_count }} suspensions for tracked accounts.
{{ untracked_suspensions | length }} accounts have more than {{ untracked_suspensions_limit }} followers and are included here.
  ✔️ indicates that the account was verified and 🔒 that it was locked.

<table>
    <tr>
        <th></th>
        <th align="left">Screen name</th>
        <th align="left">Created</th>
        <th align="left">Reversed</th>
        <th align="left">Status</th>
        <th align="left">Followers</th>
    </tr>
    {%- for item in untracked_suspensions %}
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id={{ item.record.user_id }}">
                <img src="{{ item.image_url }}" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/{{ item.record.screen_name }}">{{ item.record.screen_name }}</a>
                {%- if item.other_screen_name_count == 0 -%}
                {%- else -%}
                    &nbsp;(<a href="https://memory.lol/tw/id/{{ item.record.user_id }}">
                        {{- item.other_screen_name_count }} other{{ item.other_screen_name_count | pluralize -}}
                    </a>)&nbsp;
                {%- endif -%}
            </td>
            <td>{{ item.record.created_at | date(format="%Y-%m-%d") }}</td>
            <td>{% if item.record.reversal %}{{ item.record.reversal | date(format="%Y-%m-%d") }}{% else %}{% endif %}</td>
            <td align="center">
                {%- if item.record.protected == 0 %}🔒{% else %}{% endif %}{% if item.record.verified == 0 -%}✔️{%- else -%}{%- endif -%}
            </td>
            <td>{{ item.record.followers_count }}</td>
        </tr>
    {%- endfor -%}
</table>
