from rest_framework import serializers
from .models import Net
from django.contrib.auth.models import User

class NetSerializer(serializers.ModelSerializer):
    class Meta:
        model = Net
        fields = '__all__'

class UserSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = [
                'id',
                'username',
                'email',
                ]
