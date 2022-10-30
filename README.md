[![Rust build status](https://img.shields.io/github/workflow/status/travisbrown/twitter-watch/rust-ci.svg?label=rust)](https://github.com/travisbrown/twitter-watch/actions)

# Twitter reports

* [Overview](#overview)
* [Top 20 suspensions](#top-20-suspensions)
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

The third table only includes suspended _untracked_ accounts with more than 1000 followers.
These are often bots or spam accounts.

Suspension (and suspension reversal) dates indicate when the change in account status was detected,
which in some cases may be up to several days after the change occurred.

![Chart of total daily suspension counts](charts/suspensions.svg)
![Chart of total daily screen name change counts](charts/screen-names.svg)

## Overview

* Total number of suspensions detected: 316640
* Total number of suspension reversals detected: 6904
* Mean number of followers for suspended accounts: 1843.34
* Median number of followers for suspended accounts: 62.0
* Mean age of suspended accounts (days): 1052.08
* Median age of suspended accounts (days): 318.0
* Total number of verified accounts suspended: 116
* Total number of protected accounts suspended: 15589
* Total number of suspensions for accounts previously withheld in specific countries: 23
* Total number of screen name changes detected: 756651



## Top 20 suspensions

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
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=16703058">
                <img src="https://pbs.twimg.com/profile_images/1112759817/BraveLad_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/BraveLad">BraveLad</a></td>
            <td>2008-10-12</td>
            <td>2022-10-29</td>
            <td>471531</td>
            <td>15</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1226624031367073793">
                <img src="https://pbs.twimg.com/profile_images/1375957476294717440/tVsfE0P8_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/ASBMilitary">ASBMilitary</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/1226624031367073793">1 other</a>)&nbsp;</td>
            <td>2020-02-09</td>
            <td>2022-03-11</td>
            <td>198960</td>
            <td>56</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=826261914">
                <img src="https://pbs.twimg.com/profile_images/1459175734602350593/cW3fs5lR_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/ConceptualJames">ConceptualJames</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/826261914">1 other</a>)&nbsp;</td>
            <td>2012-09-16</td>
            <td>2022-08-05</td>
            <td>315834</td>
            <td>65</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=2244134066">
                <img src="https://pbs.twimg.com/profile_images/378800000867032148/JdKa3H8o_normal.jpeg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/clif_high">clif_high</a></td>
            <td>2013-12-13</td>
            <td>2022-03-30</td>
            <td>105607</td>
            <td>107</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1029465832781492226">
                <img src="https://pbs.twimg.com/profile_images/1121831830586568704/qLp2jM8U_normal.png" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/RekietaLaw">RekietaLaw</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/1029465832781492226">2 others</a>)&nbsp;</td>
            <td>2018-08-14</td>
            <td>2022-09-27</td>
            <td>124312</td>
            <td>124</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=548258435">
                <img src="https://pbs.twimg.com/profile_images/1481021073181265922/TBVzFzJZ_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/wigger">wigger</a></td>
            <td>2012-04-08</td>
            <td>2022-04-30</td>
            <td>99749</td>
            <td>144</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1435906191373180933">
                <img src="https://pbs.twimg.com/profile_images/1435910607509680129/DPULcW_5_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/RealPepeEscobar">RealPepeEscobar</a></td>
            <td>2021-09-09</td>
            <td>2022-04-13</td>
            <td>78570</td>
            <td>149</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1393240605828993028">
                <img src="https://pbs.twimg.com/profile_images/1549607701851807744/Ig2Z6_ar_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/crackconnoisser">crackconnoisser</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/1393240605828993028">1 other</a>)&nbsp;</td>
            <td>2021-05-14</td>
            <td>2022-08-21</td>
            <td>80314</td>
            <td>171</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=29658469">
                <img src="https://pbs.twimg.com/profile_images/1531568430943334400/jPg3_RH5_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/MillerStream">MillerStream</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/29658469">1 other</a>)&nbsp;</td>
            <td>2009-04-08</td>
            <td>2022-06-25</td>
            <td>95603</td>
            <td>173</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1563536866070392832">
                <img src="https://abs.twimg.com/sticky/default_profile_images/default_profile_normal.png" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/BenJohn97534340">BenJohn97534340</a></td>
            <td>2022-08-27</td>
            <td>2022-10-11</td>
            <td>83666</td>
            <td>174</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=2871659160">
                <img src="https://pbs.twimg.com/profile_images/1540570428938264579/ZU8HOYq__normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/isabellarileyus">isabellarileyus</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/2871659160">2 others</a>)&nbsp;</td>
            <td>2014-10-22</td>
            <td>2022-07-10</td>
            <td>82381</td>
            <td>194</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1161154266049986560">
                <img src="https://pbs.twimg.com/profile_images/1483494731732692999/lAEpflX2_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/America1Scotty">America1Scotty</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/1161154266049986560">2 others</a>)&nbsp;</td>
            <td>2019-08-13</td>
            <td>2022-09-01</td>
            <td>81523</td>
            <td>197</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1517417467529805826">
                <img src="https://pbs.twimg.com/profile_images/1517457391717130240/JaAYWZhZ_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/GonzaloLira1968">GonzaloLira1968</a></td>
            <td>2022-04-22</td>
            <td>2022-09-13</td>
            <td>69860</td>
            <td>202</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=6811832">
                <img src="https://pbs.twimg.com/profile_images/1576202862706790400/J9FAMTAz_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/DrKarlynB">DrKarlynB</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/6811832">2 others</a>)&nbsp;</td>
            <td>2007-06-14</td>
            <td>2022-08-19</td>
            <td>69470</td>
            <td>206</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=4764958742">
                <img src="https://pbs.twimg.com/profile_images/1241698593330860033/MrHmJScY_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/QuarantinedCoof">QuarantinedCoof</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/4764958742">1 other</a>)&nbsp;</td>
            <td>2016-01-15</td>
            <td>2022-09-02</td>
            <td>70298</td>
            <td>207</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1153042794375712770">
                <img src="https://pbs.twimg.com/profile_images/1520223898285002752/lpxCWbcc_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/HowleyReporter">HowleyReporter</a></td>
            <td>2019-07-21</td>
            <td>2022-05-06</td>
            <td>74933</td>
            <td>215</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1057187257558413312">
                <img src="https://pbs.twimg.com/profile_images/1503581588663291907/Yj3M1zq3_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/realGonzaloLira">realGonzaloLira</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/1057187257558413312">2 others</a>)&nbsp;</td>
            <td>2018-10-30</td>
            <td>2022-09-16</td>
            <td>59938</td>
            <td>217</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=3243718433">
                <img src="https://pbs.twimg.com/profile_images/1518713970965860354/_VVhwraM_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/Youblacksoul">Youblacksoul</a></td>
            <td>2015-05-09</td>
            <td>2022-06-16</td>
            <td>50107</td>
            <td>227</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1416678409908928513">
                <img src="https://pbs.twimg.com/profile_images/1475008136947847168/cZG93lGn_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/DJTTracker">DJTTracker</a>&nbsp;(<a href="https://api.memory.lol/v1/tw/id/1416678409908928513">1 other</a>)&nbsp;</td>
            <td>2021-07-18</td>
            <td>2022-07-16</td>
            <td>62859</td>
            <td>235</td>
        </tr>
        <tr>
            <td><a href="https://twitter.com/intent/user?user_id=1497711967091535873">
                <img src="https://pbs.twimg.com/profile_images/1504232702844747782/2wN79C77_normal.jpg" width="40px" height="40px" align="center"/></a>
            </td>
            <td>
                <a href="https://twitter.com/MapsUkraine">MapsUkraine</a></td>
            <td>2022-02-26</td>
            <td>2022-04-23</td>
            <td>46370</td>
            <td>239</td>
        </tr></table>

## Daily reports

<table>
    <tr>
        <th align="left">Date</th>
        <th align="left">Total suspensions</th>
        <th align="left">Tracked suspensions</th>
        <th align="left">Tracked screen name changes</th>
    </tr>
        <tr>
            <td>
                <a href="reports/2022-10-30/">30 October 2022</a>
            </td>
            <td>2206</td>
            <td>354</td>
            <td>15</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-29/">29 October 2022</a>
            </td>
            <td>12714</td>
            <td>45</td>
            <td>53</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-28/">28 October 2022</a>
            </td>
            <td>2257</td>
            <td>54</td>
            <td>68</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-27/">27 October 2022</a>
            </td>
            <td>1459</td>
            <td>43</td>
            <td>43</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-26/">26 October 2022</a>
            </td>
            <td>1976</td>
            <td>24</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-25/">25 October 2022</a>
            </td>
            <td>1400</td>
            <td>12</td>
            <td>37</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-24/">24 October 2022</a>
            </td>
            <td>1140</td>
            <td>11</td>
            <td>39</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-23/">23 October 2022</a>
            </td>
            <td>1310</td>
            <td>1</td>
            <td>41</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-22/">22 October 2022</a>
            </td>
            <td>1513</td>
            <td>6</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-21/">21 October 2022</a>
            </td>
            <td>1408</td>
            <td>10</td>
            <td>52</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-20/">20 October 2022</a>
            </td>
            <td>7375</td>
            <td>14</td>
            <td>39</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-19/">19 October 2022</a>
            </td>
            <td>3140</td>
            <td>13</td>
            <td>36</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-18/">18 October 2022</a>
            </td>
            <td>1618</td>
            <td>4</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-17/">17 October 2022</a>
            </td>
            <td>1306</td>
            <td>5</td>
            <td>26</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-16/">16 October 2022</a>
            </td>
            <td>1000</td>
            <td>1</td>
            <td>28</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-15/">15 October 2022</a>
            </td>
            <td>1707</td>
            <td>1</td>
            <td>86</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-14/">14 October 2022</a>
            </td>
            <td>2142</td>
            <td>16</td>
            <td>0</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-13/">13 October 2022</a>
            </td>
            <td>1850</td>
            <td>4</td>
            <td>11</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-12/">12 October 2022</a>
            </td>
            <td>2006</td>
            <td>10</td>
            <td>40</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-11/">11 October 2022</a>
            </td>
            <td>1711</td>
            <td>5</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-10/">10 October 2022</a>
            </td>
            <td>1581</td>
            <td>5</td>
            <td>30</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-09/"> 9 October 2022</a>
            </td>
            <td>1126</td>
            <td>3</td>
            <td>37</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-08/"> 8 October 2022</a>
            </td>
            <td>1555</td>
            <td>2</td>
            <td>22</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-07/"> 7 October 2022</a>
            </td>
            <td>1438</td>
            <td>7</td>
            <td>31</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-06/"> 6 October 2022</a>
            </td>
            <td>1686</td>
            <td>12</td>
            <td>27</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-05/"> 5 October 2022</a>
            </td>
            <td>1614</td>
            <td>6</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-04/"> 4 October 2022</a>
            </td>
            <td>1390</td>
            <td>4</td>
            <td>31</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-03/"> 3 October 2022</a>
            </td>
            <td>1454</td>
            <td>6</td>
            <td>27</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-02/"> 2 October 2022</a>
            </td>
            <td>1098</td>
            <td>4</td>
            <td>28</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-10-01/"> 1 October 2022</a>
            </td>
            <td>1719</td>
            <td>6</td>
            <td>53</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-30/">30 September 2022</a>
            </td>
            <td>1594</td>
            <td>17</td>
            <td>5</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-29/">29 September 2022</a>
            </td>
            <td>1715</td>
            <td>6</td>
            <td>36</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-28/">28 September 2022</a>
            </td>
            <td>1617</td>
            <td>8</td>
            <td>56</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-27/">27 September 2022</a>
            </td>
            <td>1062</td>
            <td>2</td>
            <td>10</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-26/">26 September 2022</a>
            </td>
            <td>1521</td>
            <td>12</td>
            <td>53</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-25/">25 September 2022</a>
            </td>
            <td>1455</td>
            <td>3</td>
            <td>5</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-24/">24 September 2022</a>
            </td>
            <td>1656</td>
            <td>4</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-23/">23 September 2022</a>
            </td>
            <td>5345</td>
            <td>4</td>
            <td>37</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-22/">22 September 2022</a>
            </td>
            <td>2333</td>
            <td>6</td>
            <td>42</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-21/">21 September 2022</a>
            </td>
            <td>4821</td>
            <td>10</td>
            <td>31</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-20/">20 September 2022</a>
            </td>
            <td>2861</td>
            <td>7</td>
            <td>36</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-19/">19 September 2022</a>
            </td>
            <td>1847</td>
            <td>5</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-18/">18 September 2022</a>
            </td>
            <td>2703</td>
            <td>2</td>
            <td>29</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-17/">17 September 2022</a>
            </td>
            <td>1909</td>
            <td>4</td>
            <td>39</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-16/">16 September 2022</a>
            </td>
            <td>2195</td>
            <td>22</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-15/">15 September 2022</a>
            </td>
            <td>1463</td>
            <td>4</td>
            <td>22</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-14/">14 September 2022</a>
            </td>
            <td>1625</td>
            <td>5</td>
            <td>25</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-13/">13 September 2022</a>
            </td>
            <td>2052</td>
            <td>6</td>
            <td>30</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-12/">12 September 2022</a>
            </td>
            <td>1593</td>
            <td>4</td>
            <td>31</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-11/">11 September 2022</a>
            </td>
            <td>1225</td>
            <td>2</td>
            <td>30</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-10/">10 September 2022</a>
            </td>
            <td>2066</td>
            <td>5</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-09/"> 9 September 2022</a>
            </td>
            <td>2021</td>
            <td>1</td>
            <td>41</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-08/"> 8 September 2022</a>
            </td>
            <td>2071</td>
            <td>7</td>
            <td>18</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-07/"> 7 September 2022</a>
            </td>
            <td>1104</td>
            <td>9</td>
            <td>27</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-06/"> 6 September 2022</a>
            </td>
            <td>391</td>
            <td>6</td>
            <td>25</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-05/"> 5 September 2022</a>
            </td>
            <td>940</td>
            <td>4</td>
            <td>22</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-04/"> 4 September 2022</a>
            </td>
            <td>969</td>
            <td>13</td>
            <td>29</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-03/"> 3 September 2022</a>
            </td>
            <td>1311</td>
            <td>22</td>
            <td>32</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-02/"> 2 September 2022</a>
            </td>
            <td>1481</td>
            <td>9</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-09-01/"> 1 September 2022</a>
            </td>
            <td>1517</td>
            <td>4</td>
            <td>40</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-31/">31 August 2022</a>
            </td>
            <td>1217</td>
            <td>5</td>
            <td>52</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-30/">30 August 2022</a>
            </td>
            <td>1534</td>
            <td>9</td>
            <td>6</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-29/">29 August 2022</a>
            </td>
            <td>1437</td>
            <td>6</td>
            <td>26</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-28/">28 August 2022</a>
            </td>
            <td>1246</td>
            <td>0</td>
            <td>20</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-27/">27 August 2022</a>
            </td>
            <td>1413</td>
            <td>5</td>
            <td>28</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-26/">26 August 2022</a>
            </td>
            <td>2431</td>
            <td>8</td>
            <td>47</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-25/">25 August 2022</a>
            </td>
            <td>1711</td>
            <td>2</td>
            <td>21</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-24/">24 August 2022</a>
            </td>
            <td>1733</td>
            <td>9</td>
            <td>39</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-23/">23 August 2022</a>
            </td>
            <td>1405</td>
            <td>5</td>
            <td>42</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-22/">22 August 2022</a>
            </td>
            <td>1825</td>
            <td>5</td>
            <td>24</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-21/">21 August 2022</a>
            </td>
            <td>1206</td>
            <td>13</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-20/">20 August 2022</a>
            </td>
            <td>1572</td>
            <td>9</td>
            <td>36</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-19/">19 August 2022</a>
            </td>
            <td>1987</td>
            <td>10</td>
            <td>32</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-18/">18 August 2022</a>
            </td>
            <td>1295</td>
            <td>9</td>
            <td>24</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-17/">17 August 2022</a>
            </td>
            <td>1628</td>
            <td>16</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-16/">16 August 2022</a>
            </td>
            <td>1075</td>
            <td>3</td>
            <td>28</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-15/">15 August 2022</a>
            </td>
            <td>1168</td>
            <td>7</td>
            <td>40</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-14/">14 August 2022</a>
            </td>
            <td>845</td>
            <td>9</td>
            <td>38</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-13/">13 August 2022</a>
            </td>
            <td>1345</td>
            <td>4</td>
            <td>47</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-12/">12 August 2022</a>
            </td>
            <td>965</td>
            <td>9</td>
            <td>19</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-11/">11 August 2022</a>
            </td>
            <td>1260</td>
            <td>8</td>
            <td>79</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-10/">10 August 2022</a>
            </td>
            <td>1767</td>
            <td>14</td>
            <td>31</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-09/"> 9 August 2022</a>
            </td>
            <td>418</td>
            <td>7</td>
            <td>37</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-08/"> 8 August 2022</a>
            </td>
            <td>1263</td>
            <td>7</td>
            <td>37</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-07/"> 7 August 2022</a>
            </td>
            <td>1115</td>
            <td>5</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-06/"> 6 August 2022</a>
            </td>
            <td>812</td>
            <td>5</td>
            <td>46</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-05/"> 5 August 2022</a>
            </td>
            <td>901</td>
            <td>8</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-04/"> 4 August 2022</a>
            </td>
            <td>1394</td>
            <td>6</td>
            <td>31</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-03/"> 3 August 2022</a>
            </td>
            <td>998</td>
            <td>6</td>
            <td>29</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-02/"> 2 August 2022</a>
            </td>
            <td>1039</td>
            <td>11</td>
            <td>10</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-08-01/"> 1 August 2022</a>
            </td>
            <td>930</td>
            <td>2</td>
            <td>32</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-31/">31 July 2022</a>
            </td>
            <td>1079</td>
            <td>6</td>
            <td>37</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-30/">30 July 2022</a>
            </td>
            <td>1167</td>
            <td>5</td>
            <td>8</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-29/">29 July 2022</a>
            </td>
            <td>1081</td>
            <td>8</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-28/">28 July 2022</a>
            </td>
            <td>2807</td>
            <td>10</td>
            <td>41</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-27/">27 July 2022</a>
            </td>
            <td>2155</td>
            <td>3</td>
            <td>39</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-26/">26 July 2022</a>
            </td>
            <td>2889</td>
            <td>7</td>
            <td>14</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-25/">25 July 2022</a>
            </td>
            <td>1747</td>
            <td>7</td>
            <td>38</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-24/">24 July 2022</a>
            </td>
            <td>1034</td>
            <td>6</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-23/">23 July 2022</a>
            </td>
            <td>1539</td>
            <td>8</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-22/">22 July 2022</a>
            </td>
            <td>1383</td>
            <td>6</td>
            <td>63</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-21/">21 July 2022</a>
            </td>
            <td>1519</td>
            <td>11</td>
            <td>3</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-20/">20 July 2022</a>
            </td>
            <td>1385</td>
            <td>7</td>
            <td>22</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-19/">19 July 2022</a>
            </td>
            <td>1093</td>
            <td>5</td>
            <td>45</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-18/">18 July 2022</a>
            </td>
            <td>1544</td>
            <td>10</td>
            <td>71</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-17/">17 July 2022</a>
            </td>
            <td>2025</td>
            <td>2</td>
            <td>7</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-16/">16 July 2022</a>
            </td>
            <td>2469</td>
            <td>7</td>
            <td>8</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-15/">15 July 2022</a>
            </td>
            <td>2268</td>
            <td>22</td>
            <td>12</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-14/">14 July 2022</a>
            </td>
            <td>1203</td>
            <td>7</td>
            <td>65</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-13/">13 July 2022</a>
            </td>
            <td>2381</td>
            <td>6</td>
            <td>6</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-12/">12 July 2022</a>
            </td>
            <td>1397</td>
            <td>5</td>
            <td>61</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-11/">11 July 2022</a>
            </td>
            <td>2974</td>
            <td>3</td>
            <td>8</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-10/">10 July 2022</a>
            </td>
            <td>430</td>
            <td>5</td>
            <td>46</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-09/"> 9 July 2022</a>
            </td>
            <td>681</td>
            <td>10</td>
            <td>19</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-08/"> 8 July 2022</a>
            </td>
            <td>753</td>
            <td>5</td>
            <td>49</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-07/"> 7 July 2022</a>
            </td>
            <td>1100</td>
            <td>31</td>
            <td>50</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-06/"> 6 July 2022</a>
            </td>
            <td>861</td>
            <td>9</td>
            <td>19</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-05/"> 5 July 2022</a>
            </td>
            <td>783</td>
            <td>4</td>
            <td>28</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-04/"> 4 July 2022</a>
            </td>
            <td>622</td>
            <td>9</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-07-03/"> 3 July 2022</a>
            </td>
            <td>3820</td>
            <td>15</td>
            <td>168</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-29/">29 June 2022</a>
            </td>
            <td>1820</td>
            <td>11</td>
            <td>8</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-28/">28 June 2022</a>
            </td>
            <td>1962</td>
            <td>49</td>
            <td>36</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-27/">27 June 2022</a>
            </td>
            <td>771</td>
            <td>5</td>
            <td>54</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-26/">26 June 2022</a>
            </td>
            <td>1406</td>
            <td>7</td>
            <td>23</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-25/">25 June 2022</a>
            </td>
            <td>1423</td>
            <td>10</td>
            <td>50</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-24/">24 June 2022</a>
            </td>
            <td>667</td>
            <td>4</td>
            <td>53</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-23/">23 June 2022</a>
            </td>
            <td>1639</td>
            <td>5</td>
            <td>4</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-22/">22 June 2022</a>
            </td>
            <td>763</td>
            <td>3</td>
            <td>69</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-21/">21 June 2022</a>
            </td>
            <td>864</td>
            <td>5</td>
            <td>5</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-20/">20 June 2022</a>
            </td>
            <td>981</td>
            <td>10</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-19/">19 June 2022</a>
            </td>
            <td>1378</td>
            <td>2</td>
            <td>17</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-18/">18 June 2022</a>
            </td>
            <td>831</td>
            <td>1</td>
            <td>72</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-17/">17 June 2022</a>
            </td>
            <td>1688</td>
            <td>6</td>
            <td>7</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-16/">16 June 2022</a>
            </td>
            <td>358</td>
            <td>13</td>
            <td>60</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-15/">15 June 2022</a>
            </td>
            <td>2089</td>
            <td>9</td>
            <td>11</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-14/">14 June 2022</a>
            </td>
            <td>1227</td>
            <td>20</td>
            <td>20</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-13/">13 June 2022</a>
            </td>
            <td>1614</td>
            <td>9</td>
            <td>87</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-12/">12 June 2022</a>
            </td>
            <td>1630</td>
            <td>12</td>
            <td>8</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-10/">10 June 2022</a>
            </td>
            <td>1077</td>
            <td>1</td>
            <td>23</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-09/"> 9 June 2022</a>
            </td>
            <td>724</td>
            <td>9</td>
            <td>52</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-08/"> 8 June 2022</a>
            </td>
            <td>795</td>
            <td>37</td>
            <td>12</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-07/"> 7 June 2022</a>
            </td>
            <td>818</td>
            <td>19</td>
            <td>89</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-06/"> 6 June 2022</a>
            </td>
            <td>880</td>
            <td>31</td>
            <td>4</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-05/"> 5 June 2022</a>
            </td>
            <td>1244</td>
            <td>1</td>
            <td>7</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-04/"> 4 June 2022</a>
            </td>
            <td>1079</td>
            <td>53</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-03/"> 3 June 2022</a>
            </td>
            <td>532</td>
            <td>59</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-02/"> 2 June 2022</a>
            </td>
            <td>1113</td>
            <td>20</td>
            <td>49</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-06-01/"> 1 June 2022</a>
            </td>
            <td>1596</td>
            <td>31</td>
            <td>61</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-31/">31 May 2022</a>
            </td>
            <td>692</td>
            <td>17</td>
            <td>24</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-30/">30 May 2022</a>
            </td>
            <td>709</td>
            <td>3</td>
            <td>5</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-29/">29 May 2022</a>
            </td>
            <td>776</td>
            <td>4</td>
            <td>64</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-28/">28 May 2022</a>
            </td>
            <td>907</td>
            <td>7</td>
            <td>15</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-27/">27 May 2022</a>
            </td>
            <td>840</td>
            <td>11</td>
            <td>32</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-26/">26 May 2022</a>
            </td>
            <td>969</td>
            <td>9</td>
            <td>41</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-25/">25 May 2022</a>
            </td>
            <td>2320</td>
            <td>8</td>
            <td>37</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-24/">24 May 2022</a>
            </td>
            <td>742</td>
            <td>5</td>
            <td>59</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-23/">23 May 2022</a>
            </td>
            <td>1012</td>
            <td>1</td>
            <td>8</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-22/">22 May 2022</a>
            </td>
            <td>733</td>
            <td>4</td>
            <td>43</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-21/">21 May 2022</a>
            </td>
            <td>1068</td>
            <td>3</td>
            <td>68</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-20/">20 May 2022</a>
            </td>
            <td>811</td>
            <td>4</td>
            <td>8</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-19/">19 May 2022</a>
            </td>
            <td>892</td>
            <td>3</td>
            <td>69</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-18/">18 May 2022</a>
            </td>
            <td>822</td>
            <td>5</td>
            <td>4</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-17/">17 May 2022</a>
            </td>
            <td>631</td>
            <td>3</td>
            <td>38</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-16/">16 May 2022</a>
            </td>
            <td>1398</td>
            <td>3</td>
            <td>46</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-15/">15 May 2022</a>
            </td>
            <td>525</td>
            <td>4</td>
            <td>11</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-14/">14 May 2022</a>
            </td>
            <td>925</td>
            <td>5</td>
            <td>43</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-13/">13 May 2022</a>
            </td>
            <td>1116</td>
            <td>7</td>
            <td>63</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-12/">12 May 2022</a>
            </td>
            <td>941</td>
            <td>4</td>
            <td>4</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-11/">11 May 2022</a>
            </td>
            <td>557</td>
            <td>1</td>
            <td>63</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-10/">10 May 2022</a>
            </td>
            <td>1178</td>
            <td>6</td>
            <td>6</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-09/"> 9 May 2022</a>
            </td>
            <td>1172</td>
            <td>10</td>
            <td>53</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-08/"> 8 May 2022</a>
            </td>
            <td>519</td>
            <td>2</td>
            <td>52</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-07/"> 7 May 2022</a>
            </td>
            <td>679</td>
            <td>2</td>
            <td>48</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-06/"> 6 May 2022</a>
            </td>
            <td>1437</td>
            <td>10</td>
            <td>28</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-05/"> 5 May 2022</a>
            </td>
            <td>1300</td>
            <td>16</td>
            <td>92</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-04/"> 4 May 2022</a>
            </td>
            <td>927</td>
            <td>2</td>
            <td>32</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-03/"> 3 May 2022</a>
            </td>
            <td>1291</td>
            <td>6</td>
            <td>36</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-02/"> 2 May 2022</a>
            </td>
            <td>867</td>
            <td>5</td>
            <td>37</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-05-01/"> 1 May 2022</a>
            </td>
            <td>676</td>
            <td>2</td>
            <td>40</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-30/">30 April 2022</a>
            </td>
            <td>1621</td>
            <td>5</td>
            <td>163</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-29/">29 April 2022</a>
            </td>
            <td>519</td>
            <td>7</td>
            <td>7</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-28/">28 April 2022</a>
            </td>
            <td>765</td>
            <td>0</td>
            <td>15</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-27/">27 April 2022</a>
            </td>
            <td>1064</td>
            <td>8</td>
            <td>146</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-26/">26 April 2022</a>
            </td>
            <td>481</td>
            <td>4</td>
            <td>10</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-25/">25 April 2022</a>
            </td>
            <td>1036</td>
            <td>5</td>
            <td>204</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-24/">24 April 2022</a>
            </td>
            <td>3915</td>
            <td>3</td>
            <td>12</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-23/">23 April 2022</a>
            </td>
            <td>1294</td>
            <td>30</td>
            <td>3</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-18/">18 April 2022</a>
            </td>
            <td>150</td>
            <td>1</td>
            <td>25</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-17/">17 April 2022</a>
            </td>
            <td>1055</td>
            <td>4</td>
            <td>78</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-16/">16 April 2022</a>
            </td>
            <td>323</td>
            <td>0</td>
            <td>0</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-15/">15 April 2022</a>
            </td>
            <td>1205</td>
            <td>4</td>
            <td>92</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-14/">14 April 2022</a>
            </td>
            <td>532</td>
            <td>3</td>
            <td>5</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-13/">13 April 2022</a>
            </td>
            <td>662</td>
            <td>8</td>
            <td>3</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-12/">12 April 2022</a>
            </td>
            <td>636</td>
            <td>9</td>
            <td>46</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-11/">11 April 2022</a>
            </td>
            <td>751</td>
            <td>2</td>
            <td>6</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-10/">10 April 2022</a>
            </td>
            <td>798</td>
            <td>9</td>
            <td>36</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-09/"> 9 April 2022</a>
            </td>
            <td>968</td>
            <td>4</td>
            <td>30</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-08/"> 8 April 2022</a>
            </td>
            <td>643</td>
            <td>4</td>
            <td>51</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-07/"> 7 April 2022</a>
            </td>
            <td>836</td>
            <td>4</td>
            <td>43</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-06/"> 6 April 2022</a>
            </td>
            <td>719</td>
            <td>3</td>
            <td>68</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-05/"> 5 April 2022</a>
            </td>
            <td>1640</td>
            <td>7</td>
            <td>32</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-04/"> 4 April 2022</a>
            </td>
            <td>1026</td>
            <td>7</td>
            <td>41</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-03/"> 3 April 2022</a>
            </td>
            <td>1000</td>
            <td>3</td>
            <td>8</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-02/"> 2 April 2022</a>
            </td>
            <td>881</td>
            <td>4</td>
            <td>29</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-04-01/"> 1 April 2022</a>
            </td>
            <td>603</td>
            <td>1</td>
            <td>16</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-31/">31 March 2022</a>
            </td>
            <td>872</td>
            <td>6</td>
            <td>44</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-30/">30 March 2022</a>
            </td>
            <td>1004</td>
            <td>7</td>
            <td>52</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-29/">29 March 2022</a>
            </td>
            <td>862</td>
            <td>0</td>
            <td>29</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-28/">28 March 2022</a>
            </td>
            <td>565</td>
            <td>1</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-27/">27 March 2022</a>
            </td>
            <td>548</td>
            <td>4</td>
            <td>43</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-26/">26 March 2022</a>
            </td>
            <td>793</td>
            <td>4</td>
            <td>31</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-25/">25 March 2022</a>
            </td>
            <td>871</td>
            <td>1</td>
            <td>18</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-24/">24 March 2022</a>
            </td>
            <td>881</td>
            <td>5</td>
            <td>38</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-23/">23 March 2022</a>
            </td>
            <td>895</td>
            <td>7</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-22/">22 March 2022</a>
            </td>
            <td>1038</td>
            <td>4</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-21/">21 March 2022</a>
            </td>
            <td>534</td>
            <td>4</td>
            <td>36</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-20/">20 March 2022</a>
            </td>
            <td>750</td>
            <td>3</td>
            <td>23</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-19/">19 March 2022</a>
            </td>
            <td>1134</td>
            <td>5</td>
            <td>34</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-18/">18 March 2022</a>
            </td>
            <td>526</td>
            <td>0</td>
            <td>46</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-17/">17 March 2022</a>
            </td>
            <td>1039</td>
            <td>8</td>
            <td>42</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-16/">16 March 2022</a>
            </td>
            <td>1265</td>
            <td>4</td>
            <td>16</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-15/">15 March 2022</a>
            </td>
            <td>824</td>
            <td>7</td>
            <td>38</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-14/">14 March 2022</a>
            </td>
            <td>1279</td>
            <td>5</td>
            <td>43</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-13/">13 March 2022</a>
            </td>
            <td>632</td>
            <td>1</td>
            <td>22</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-12/">12 March 2022</a>
            </td>
            <td>439</td>
            <td>2</td>
            <td>31</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-11/">11 March 2022</a>
            </td>
            <td>868</td>
            <td>8</td>
            <td>35</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-10/">10 March 2022</a>
            </td>
            <td>508</td>
            <td>0</td>
            <td>18</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-09/"> 9 March 2022</a>
            </td>
            <td>927</td>
            <td>1</td>
            <td>50</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-08/"> 8 March 2022</a>
            </td>
            <td>514</td>
            <td>12</td>
            <td>28</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-07/"> 7 March 2022</a>
            </td>
            <td>653</td>
            <td>2</td>
            <td>38</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-06/"> 6 March 2022</a>
            </td>
            <td>641</td>
            <td>1</td>
            <td>32</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-05/"> 5 March 2022</a>
            </td>
            <td>556</td>
            <td>1</td>
            <td>40</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-04/"> 4 March 2022</a>
            </td>
            <td>545</td>
            <td>2</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-03/"> 3 March 2022</a>
            </td>
            <td>776</td>
            <td>2</td>
            <td>39</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-02/"> 2 March 2022</a>
            </td>
            <td>1215</td>
            <td>1</td>
            <td>51</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-03-01/"> 1 March 2022</a>
            </td>
            <td>709</td>
            <td>5</td>
            <td>27</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-28/">28 February 2022</a>
            </td>
            <td>607</td>
            <td>6</td>
            <td>48</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-27/">27 February 2022</a>
            </td>
            <td>279</td>
            <td>0</td>
            <td>30</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-26/">26 February 2022</a>
            </td>
            <td>866</td>
            <td>2</td>
            <td>39</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-25/">25 February 2022</a>
            </td>
            <td>566</td>
            <td>2</td>
            <td>26</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-24/">24 February 2022</a>
            </td>
            <td>285</td>
            <td>2</td>
            <td>33</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-23/">23 February 2022</a>
            </td>
            <td>500</td>
            <td>1</td>
            <td>50</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-22/">22 February 2022</a>
            </td>
            <td>615</td>
            <td>2</td>
            <td>42</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-21/">21 February 2022</a>
            </td>
            <td>419</td>
            <td>6</td>
            <td>45</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-20/">20 February 2022</a>
            </td>
            <td>1241</td>
            <td>0</td>
            <td>63</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-19/">19 February 2022</a>
            </td>
            <td>561</td>
            <td>5</td>
            <td>27</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-18/">18 February 2022</a>
            </td>
            <td>505</td>
            <td>0</td>
            <td>50</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-17/">17 February 2022</a>
            </td>
            <td>206</td>
            <td>1</td>
            <td>51</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-16/">16 February 2022</a>
            </td>
            <td>673</td>
            <td>5</td>
            <td>50</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-15/">15 February 2022</a>
            </td>
            <td>702</td>
            <td>1</td>
            <td>129</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-14/">14 February 2022</a>
            </td>
            <td>1028</td>
            <td>1</td>
            <td>3</td>
        </tr>
        <tr>
            <td>
                <a href="reports/2022-02-13/">13 February 2022</a>
            </td>
            <td>3741</td>
            <td>8</td>
            <td>1</td>
        </tr></table>

## License

The software in this repository (currently only report formatting code) is published under the [Anti-Capitalist Software License][acsl] (v. 1.4).

[acsl]: https://anticapitalist.software/
