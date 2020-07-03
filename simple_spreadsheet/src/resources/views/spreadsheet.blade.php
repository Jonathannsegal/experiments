<head>
    <title>{{ $sheet->name }}</title>
    <meta name="csrf-token" content="{{ csrf_token() }}">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="https://handsontable.com/static/css/main.css">
    <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/handsontable@latest/dist/handsontable.full.min.css">
    <!-- Fonts -->
    <link rel="dns-prefetch" href="https://fonts.gstatic.com">
    <link href="https://fonts.googleapis.com/css?family=Raleway:300,400,600" rel="stylesheet" type="text/css">

    <!-- Styles -->
    <link href="{{ asset('css/app.css') }}" rel="stylesheet">

    <!-- Scripts -->
    <script src="https://cdn.jsdelivr.net/npm/handsontable@latest/dist/handsontable.full.min.js"></script>
</head>

<div style="height: 10vh">
    <br>
    <h2>{{ $sheet->name }}</h2>
</div>
<div id="sheet"></div>

<script>
    let csrfToken = document.head.querySelector('meta[name="csrf-token"]').content;
    let data = @json($sheet->content);

    let container = document.querySelector('#sheet');
    var containerParent = container.parentNode;
    var settings = {
        licenseKey: 'non-commercial-and-evaluation',
        data: data,
        rowHeaders: true,
        colHeaders: true,
        stretchH: 'all',
        colWidths: 100,
        autoWrapRow: true,
        height: '90vh',
        minCols: 26,
        minRows: 100,
        afterChange: function (change, source) {
            if (source === 'loadData') return;

            console.log(change, source);

            fetch('/sheets/{{ $sheet->_id }}', {
                method: 'PUT',
                body: JSON.stringify({content: data}),
                headers: {
                    'X-CSRF-TOKEN': csrfToken,
                    'Content-Type': 'application/json'
                },
                credentials: 'same-origin'
            })
        }
    };
    let table = new Handsontable(container, settings);
</script>